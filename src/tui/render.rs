use std::time::Duration;

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
                .unwrap_or_default();
            let total_bytes_send = tracker
                .get_nbr_of_bytes_send_since_monitoring_started(pid)
                .unwrap_or_default();

            Row::new([
                pid.to_string(),
                process_name.to_string(),
                send.to_string(),
                received.to_string(),
                total_bytes_send.to_string(),
                total_bytes_received.to_string(),
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
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ]);

    frame.render_widget(table, frame.size());
}
