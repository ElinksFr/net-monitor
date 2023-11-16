use bandwidth_tracker::state::BandwidthTracker;
use bpf::probs::LoadedProb;
use byte_unit::Byte;
use libbpf_rs::skel::{OpenSkel, SkelBuilder};
use procfs::process::Process;
use std::{collections::HashMap, error::Error, thread::sleep, time::Duration};

mod bandwidth_tracker;
mod bpf;
#[path = "bpf/.output/packet_size.skel.rs"]
mod packet_size;

fn main() -> Result<(), Box<dyn Error>> {
    let process_by_pid: HashMap<i32, Process> = procfs::process::all_processes()?
        .map(|process_result| {
            let process = process_result.unwrap();
            (process.pid, process)
        })
        .collect();

    let builder = packet_size::PacketSizeSkelBuilder::default();
    let opened_skel = builder.open()?;
    let mut skel = opened_skel.load()?;
    let _probs = LoadedProb::load_ebpf_monitoring_probs(&mut skel)?;

    let map_collection = skel.maps();
    let packet_stats = map_collection.packet_stats();

    let tick_rate = Duration::from_millis(200);
    let average_over = Duration::from_secs(2);

    let mut tracker = BandwidthTracker::new();
    loop {
        tracker.refresh_tick(packet_stats);
        tracker
            .get_throughput_over_duration(average_over)
            .for_each(|(pid, received, send)| {
                let bytes_since_inception =
                    tracker.get_nbr_of_bytes_received_since_monitoring_started(pid);
                print_process_throughput_info(
                    received,
                    send,
                    bytes_since_inception,
                    &process_by_pid,
                    pid,
                );
            });
        sleep(tick_rate);
        print!("{}[2J", 27 as char);
    }
}

fn print_process_throughput_info(
    bytes_per_second: u64,
    bytes_per_second_send: u64,
    bytes_since_inception: u64,
    process_by_pid: &HashMap<i32, Process>,
    pid: i32,
) {
    let pretty_bytes_troughput =
        Byte::from_bytes(bytes_per_second as u128).get_appropriate_unit(true);
    let pretty_bytes_troughput_send =
        Byte::from_bytes(bytes_per_second_send as u128).get_appropriate_unit(true);
    let pretty_bytes_total =
        Byte::from_bytes(bytes_since_inception as u128).get_appropriate_unit(true);
    match process_by_pid.get(&pid) {
        Some(process) => match process.stat() {
            Ok(stat) => {
                println!(
                    "{} | {} | {}/s | {}/s | {}",
                    process.pid,
                    stat.comm,
                    pretty_bytes_troughput,
                    pretty_bytes_troughput_send,
                    pretty_bytes_total
                )
            }
            Err(_) => (),
        },
        None => println!("Process Not Found"),
    }
}
