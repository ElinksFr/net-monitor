use byte_unit::Byte;
use libbpf_rs::{
    skel::{OpenSkel, SkelBuilder},
    MapFlags,
};
use procfs::process::Process;
use std::{collections::HashMap, error::Error, thread::sleep, time::Duration};
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

    // process_by_pid
    //     .iter()
    //     .take(15)
    //     .for_each(|(_pid, process)| match process.stat() {
    //         Ok(stat) => println!("{} | {}", process.pid, stat.comm),
    //         Err(_) => (),
    //     });

    let builder = packet_size::PacketSizeSkelBuilder::default();
    let opened_skel = builder.open()?;
    let mut skel = opened_skel.load()?;
    let _link = skel.progs_mut().packet_size().attach()?;

    let map_collection = skel.maps();
    let packet_stats = map_collection.packet_stats();

    sleep(Duration::from_secs(5));

    packet_stats.keys().into_iter().for_each(|key| {
        let pid = i32::from_le_bytes([key[0], key[1], key[2], key[3]]);

        let tmp = packet_stats
            .lookup(&key, MapFlags::ANY)
            .expect("err")
            .expect("option");
        let bytes_received = i32::from_le_bytes([tmp[0], tmp[1], tmp[2], tmp[3]]);

        match process_by_pid.get(&pid) {
            Some(process) => match process.stat() {
                Ok(stat) => println!(
                    "{} | {} | {}",
                    process.pid,
                    stat.comm,
                    Byte::from_bytes(bytes_received as u128).get_appropriate_unit(true)
                ),
                Err(_) => (),
            },
            None => println!("Process Not Found"),
        }
    });
    Ok(())
}
