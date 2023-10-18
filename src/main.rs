use std::{collections::HashMap, error::Error};

use procfs::process::Process;

fn main() -> Result<(), Box<dyn Error>> {
    let process_by_pid: HashMap<i32, Process> = procfs::process::all_processes()?
        .into_iter()
        .map(|process_result| {
            let process = process_result.unwrap();
            (process.pid, process)
        })
        .collect();

    process_by_pid
        .into_iter()
        .take(15)
        .for_each(|(_pid, process)| match process.stat() {
            Ok(stat) => println!("{} | {}", process.pid, stat.comm),
            Err(_) => (),
        });

    Ok(())
}
