use std::time::Duration;

use byte_unit::Byte;
use ratatui::{
    prelude::Constraint,
    widgets::{Row, Table},
    Frame,
};

use super::state::Model;

pub fn draw_state(frame: &mut Frame, state: &Model) {
    let tracker = &state.bandwidth_tracker;
    let rows: Vec<_> = tracker
        .get_throughput_over_duration(Duration::from_secs(5))
        .map(|(pid, received, send)| {
            let process_name = state.process_by_pid.get(&pid).unwrap().stat().unwrap().comm;
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
