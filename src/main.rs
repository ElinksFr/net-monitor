use std::{collections::HashMap, error::Error, thread::sleep, time::Duration};

use libbpf_rs::{
    skel::{OpenSkel, SkelBuilder},
    MapFlags,
};
use procfs::process::Process;
#[path = "bpf/.output/packet_size.skel.rs"]
mod packet_size;

fn main() -> Result<(), Box<dyn Error>> {
    let process_by_pid: HashMap<i32, Process> = procfs::process::all_processes()?
        .into_iter()
        .map(|process_result| {
            let process = process_result.unwrap();
            (process.pid, process)
        })
        .collect();

    process_by_pid
        .iter()
        .take(15)
        .for_each(|(_pid, process)| match process.stat() {
            Ok(stat) => println!("{} | {}", process.pid, stat.comm),
            Err(_) => (),
        });

    let builder = packet_size::PacketSizeSkelBuilder::default();
    let opened_skel = builder.open()?;
    let mut skel = opened_skel.load()?;
    let _link = skel.progs_mut().xdp_pass().attach()?;

    let map_collection = skel.maps();
    let packet_stats = map_collection.packet_stats();

    sleep(Duration::from_secs(3));

    packet_stats.keys().into_iter().for_each(|key| {
        let pid = i32::from_be_bytes([key[0], key[1], key[2], key[3]]);
        let tmp = packet_stats.lookup(&key, MapFlags::EXIST).unwrap().unwrap();
        let bytes_received = u64::from_be_bytes([
            tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7],
        ]);
        match process_by_pid.get(&pid) {
            Some(process) => match process.stat() {
                Ok(stat) => println!("{} | {} | {}", process.pid, stat.comm, bytes_received),
                Err(_) => (),
            },
            None => println!("Process Not Found"),
        }
    });
    Ok(())
}
