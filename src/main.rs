use bandwidth_tracker::tracker::BandwidthTracker;
use bpf::probs::LoadedProb;
use byte_unit::Byte;
use crossterm::{
    event::KeyEvent,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use libbpf_rs::skel::{OpenSkel, SkelBuilder};
use procfs::process::Process;
use ratatui::{
    prelude::{Constraint, CrosstermBackend},
    widgets::{Row, Table},
    Frame, Terminal,
};
use std::{collections::HashMap, error::Error, io::stdout, time::Duration};

mod bandwidth_tracker;
mod bpf;
#[path = "bpf/.output/packet_size.skel.rs"]
mod packet_size;

fn is_shutdown_event(key_event: KeyEvent) -> bool {
    if key_event.kind != crossterm::event::KeyEventKind::Press {
        false
    } else if matches!(key_event.code, crossterm::event::KeyCode::Char('q')) {
        true
    } else if matches!(key_event.code, crossterm::event::KeyCode::Char('c'))
        && matches!(key_event.modifiers, crossterm::event::KeyModifiers::CONTROL)
    {
        true
    } else {
        false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let process_by_pid: HashMap<i32, Process> = procfs::process::all_processes()?
        .map(|process_result| {
            let process = process_result.unwrap();
            (process.pid, process)
        })
        .collect();

    let opened_skel = packet_size::PacketSizeSkelBuilder::default().open()?;
    let mut skel = opened_skel.load()?;
    let _probs = LoadedProb::load_ebpf_monitoring_probs(&mut skel)?;

    let map_collection = skel.maps();
    let packet_stats = map_collection.packet_stats();
    let mut tracker = BandwidthTracker::new();

    loop {
        terminal.draw(|frame| ui(frame, &process_by_pid, &tracker))?;
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if is_shutdown_event(key) {
                    break;
                }
            }
        }
        tracker.refresh_tick(packet_stats);
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(frame: &mut Frame, process_by_pid: &HashMap<i32, Process>, tracker: &BandwidthTracker) {
    let rows: Vec<_> = tracker
        .get_throughput_over_duration(Duration::from_secs(5))
        .map(|(pid, received, send)| {
            let process_name = process_by_pid.get(&pid).unwrap().stat().unwrap().comm;
            let total_bytes_received = tracker
                .get_nbr_of_bytes_received_since_monitoring_started(pid)
                .unwrap_or(0);
            let total_bytes_send = tracker
                .get_nbr_of_bytes_send_since_monitoring_started(pid)
                .unwrap_or(0);
            let pretty_bytes_troughput_received =
                Byte::from_bytes(received as u128).get_appropriate_unit(true);
            let pretty_bytes_troughput_send =
                Byte::from_bytes(send as u128).get_appropriate_unit(true);
            let pretty_bytes_total_received: byte_unit::AdjustedByte =
                Byte::from_bytes(total_bytes_received as u128).get_appropriate_unit(true);
            let pretty_bytes_total_send: byte_unit::AdjustedByte =
                Byte::from_bytes(total_bytes_send as u128).get_appropriate_unit(true);

            Row::new([
                pid.to_string(),
                process_name.to_string(),
                pretty_bytes_troughput_send.to_string(),
                pretty_bytes_troughput_received.to_string(),
                pretty_bytes_total_send.to_string(),
                pretty_bytes_total_received.to_string(),
            ])
        })
        .collect();

    let table = Table::new(rows)
        .header(Row::new(vec![
            "pid",
            "name",
            "bytes send/s",
            "bytes received/s",
            "total bytes send",
            "total bytes received",
        ]))
        .widths(&[
            Constraint::Max(20),
            Constraint::Max(20),
            Constraint::Max(20),
            Constraint::Max(20),
            Constraint::Max(20),
            Constraint::Max(20),
        ]);

    frame.render_widget(table, frame.size());
}
