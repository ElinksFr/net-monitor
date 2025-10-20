use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    time::Duration,
};

use libbpf_rs::Map;
use procfs::process::Process;

use crate::bandwidth_tracker::tracker::BandwidthTracker;

use super::events::Event;

pub struct Model<'a> {
    pub process_by_pid: HashMap<i32, Process>,
    pub bandwidth_tracker: BandwidthTracker,
    pub datasets: BTreeMap<String, Vec<(f64, f64)>>,
    pub refresh_rate: Duration,
    packet_stats: &'a Map<'a>,
}

fn get_process_data_by_pid() -> HashMap<i32, Process> {
    procfs::process::all_processes()
        .expect("cannot read /proc")
        .filter_map(|process_result| process_result.map(|process| (process.pid, process)).ok())
        .collect()
}

impl<'a> Model<'a> {
    pub fn init(
        packet_stats: &'a Map,
        refresh_rate: Duration,
    ) -> Result<Model<'a>, Box<dyn Error>> {
        let process_by_pid = get_process_data_by_pid();
        let bandwidth_tracker = BandwidthTracker::new();

        Ok(Model {
            process_by_pid,
            bandwidth_tracker,
            packet_stats,
            datasets: BTreeMap::new(),
            refresh_rate,
        })
    }

    pub fn handel_event(mut self, event: &Event) -> Result<Model<'a>, Box<dyn Error>> {
        if event != &Event::Tick {
            return Err("Event not handeld".to_string().into());
        }

        self.bandwidth_tracker.refresh_tick(self.packet_stats);
        self.process_by_pid = get_process_data_by_pid();

        self.datasets = self
            .bandwidth_tracker
            .get_throughput_over_duration_per_interface()
            .into_iter()
            .map(|(key, values)| {
                (
                    key,
                    values
                        .iter()
                        .enumerate()
                        .map(|(n, v)| (n as f64, f64::from(*v)))
                        .collect(),
                )
            })
            .collect();
        Ok(self)
    }
}
