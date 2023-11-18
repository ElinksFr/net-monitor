use std::{collections::HashMap, error::Error};

use libbpf_rs::Map;
use procfs::{process::Process, ProcError};

use crate::bandwidth_tracker::tracker::BandwidthTracker;

use super::events::Event;

pub struct Model<'a> {
    pub process_by_pid: HashMap<i32, Process>,
    pub bandwidth_tracker: BandwidthTracker,
    packet_stats: &'a Map,
}

fn get_process_data_by_pid() -> Result<HashMap<i32, Process>, ProcError> {
    Ok(procfs::process::all_processes()?
        .map(|process_result| {
            let process = process_result.unwrap();
            (process.pid, process)
        })
        .collect())
}

impl<'a> Model<'a> {
    pub fn init(packet_stats: &Map) -> Result<Model, Box<dyn Error>> {
        let process_by_pid = get_process_data_by_pid()?;
        let bandwidth_tracker = BandwidthTracker::new();

        Ok(Model {
            process_by_pid,
            bandwidth_tracker,
            packet_stats,
        })
    }

    pub fn handel_event(mut self, event: &Event) -> Result<Model<'a>, Box<dyn Error>> {
        if event != &Event::Tick {
            return Err("Event not handeld".to_string().into());
        }

        self.bandwidth_tracker.refresh_tick(self.packet_stats);
        self.process_by_pid = get_process_data_by_pid()?;

        Ok(self)
    }
}
