use std::time::SystemTime;
use std::{collections::HashMap, time::Duration};

use libbpf_rs::{Map, MapFlags};
type PID = i32;
type NbrOfBytesSinceInception = i32;

pub struct BandwidthTracker {
    over_time_per_pid: HashMap<PID, Vec<(NbrOfBytesSinceInception, SystemTime)>>,
}

impl BandwidthTracker {
    pub fn new() -> BandwidthTracker {
        BandwidthTracker {
            over_time_per_pid: HashMap::new(),
        }
    }

    pub fn refresh_tick(&mut self, packet_stats: &Map) {
        let current_time = SystemTime::now();

        packet_stats.keys().for_each(|key| {
            let tmp = packet_stats
                .lookup(&key, MapFlags::ANY)
                .expect("err")
                .expect("option");

            let bytes_received = NbrOfBytesSinceInception::from_ne_bytes(
                tmp[..4]
                    .try_into()
                    .expect("failed to convert the value to i32"),
            );
            let pid = i32::from_ne_bytes(key.try_into().expect("failed to convert key to i32"));

            match self.over_time_per_pid.entry(pid) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().push((bytes_received, current_time));
                }
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(vec![(bytes_received, current_time)]);
                }
            }
        });
    }

    pub fn get_nbr_of_bytes_since_monitoring_started(&self, pid: PID) -> u64 {
        self.over_time_per_pid
            .get(&pid)
            .map(|ticks| ticks.last())
            .flatten()
            .map(|tick| tick.0)
            .unwrap_or(0) as u64
    }

    pub fn get_throughput_over_duration<'a>(
        &'a self,
        duration: Duration,
    ) -> impl Iterator<Item = (PID, u64)> + 'a {
        let current_time = SystemTime::now();

        self.over_time_per_pid.iter().map(move |(pid, ticks)| {
            let mut ticks_in_window = ticks
                .iter()
                .rev()
                .take_while(|(_, record_time)| *record_time + duration > current_time);

            let most_recent_tick = ticks_in_window.next();
            let oldest_tick = ticks_in_window.last();

            match (most_recent_tick, oldest_tick) {
                (Some((b1, _)), Some((b2, _))) => (*pid, (b1 - b2) as u64 / duration.as_secs()),
                (None, None) => (*pid, 0),
                (None, Some(_)) => (*pid, 0),
                (Some(_), None) => (*pid, 0),
            }
        })
    }
}
