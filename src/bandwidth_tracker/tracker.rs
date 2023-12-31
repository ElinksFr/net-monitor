use std::time::SystemTime;
use std::{collections::HashMap, time::Duration};

use libbpf_rs::{Map, MapFlags};

use super::bytes::{BytesPerSecond, NumberOfBytes};
use super::history_buffer::HistoryBuffer;
type PID = i32;

struct TrackingTick {
    received: NumberOfBytes,
    send: NumberOfBytes,
    at: SystemTime,
}

pub struct BandwidthTracker {
    last_tick: SystemTime,
    refresh_counter: u32,
    over_time_per_pid: HashMap<PID, HistoryBuffer<255, TrackingTick>>,
}

impl BandwidthTracker {
    pub fn new() -> BandwidthTracker {
        BandwidthTracker {
            last_tick: SystemTime::now(),
            refresh_counter: 0,
            over_time_per_pid: HashMap::new(),
        }
    }

    pub fn refresh_tick(&mut self, packet_stats: &Map) {
        let current_time = SystemTime::now();

        if self.refresh_counter % 10 == 0 {
            self.clear_dead_entries();
        }

        packet_stats
            .keys()
            .for_each(self.append_new_tick_to_history(packet_stats, current_time));

        self.refresh_counter += 1;
        self.last_tick = current_time;
    }

    fn append_new_tick_to_history<'a>(
        &'a mut self,
        packet_stats: &'a Map,
        current_time: SystemTime,
    ) -> impl FnMut(Vec<u8>) + 'a {
        move |key| {
            let tmp = packet_stats
                .lookup(&key, MapFlags::ANY)
                .expect("error accessing map")
                .expect("entry not found");

            let bytes_received = i32::from_ne_bytes(
                tmp[..4]
                    .try_into()
                    .expect("failed to convert bytes received the value to i32"),
            )
            .into();
            let bytes_send = i32::from_ne_bytes(
                tmp[4..]
                    .try_into()
                    .expect("failed to convert bytes send the value to i32"),
            )
            .into();
            let pid = i32::from_ne_bytes(key.try_into().expect("failed to convert key to i32"));

            let tick = TrackingTick {
                received: bytes_received,
                send: bytes_send,
                at: current_time,
            };
            match self.over_time_per_pid.entry(pid) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().push(tick);
                }
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(HistoryBuffer::init(tick));
                }
            };
        }
    }

    /// Returns `None` when the process did not interacted with the network since the monitoring started
    pub fn get_nbr_of_bytes_received_since_monitoring_started(
        &self,
        pid: PID,
    ) -> Option<NumberOfBytes> {
        self.over_time_per_pid
            .get(&pid)
            .map(|ticks| ticks.last().received)
    }

    /// Returns `None` when the process did not interacted with the network since the monitoring started
    pub fn get_nbr_of_bytes_send_since_monitoring_started(
        &self,
        pid: PID,
    ) -> Option<NumberOfBytes> {
        self.over_time_per_pid
            .get(&pid)
            .map(|ticks| ticks.last().send)
    }

    pub fn get_throughput_over_duration(
        &self,
        duration: Duration,
    ) -> impl Iterator<Item = (PID, BytesPerSecond, BytesPerSecond)> + '_ {
        let current_time = SystemTime::now();

        self.over_time_per_pid
            .iter()
            .filter(|(_pid, ticks)| ticks.last().at == self.last_tick)
            .map(move |(pid, ticks)| {
                let mut ticks_in_window = ticks
                    .into_iter()
                    .rev()
                    .take_while(|tick| tick.at + duration > current_time);

                let most_recent_tick = ticks_in_window.next();
                let oldest_tick = ticks_in_window.last();

                match (most_recent_tick, oldest_tick) {
                    (Some(t1), Some(t2)) => (
                        *pid,
                        BytesPerSecond::new(t1.received - t2.received, duration),
                        BytesPerSecond::new(t1.send - t2.send, duration),
                    ),
                    _ => (*pid, BytesPerSecond::default(), BytesPerSecond::default()),
                }
            })
    }

    fn clear_dead_entries(&mut self) {
        self.over_time_per_pid
            .retain(|_pid, buffer| buffer.last().at == self.last_tick);
    }
}
