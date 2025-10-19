use std::time::Duration;

use ratatui::{
    layout::Layout,
    prelude::Constraint,
    style::{Color, Stylize},
    text::Line,
    widgets::{Axis, Block, Chart, Dataset, GraphType, Row, Table},
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

    let table_widths = [
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ];

    let table = Table::new(rows, table_widths).header(Row::new(vec![
        "pid",
        "name",
        "bytes send/s",
        "bytes received/s",
        "total bytes send",
        "total bytes received",
    ]));

    let [top, bottom] = Layout::vertical([Constraint::Fill(1); 2]).areas(frame.area());

    let chart = get_chart_of_global_thoughputs(state);
    frame.render_widget(table, top);
    frame.render_widget(chart, bottom);
}

fn get_chart_of_global_thoughputs<'a>(state: &'a Model<'a>) -> Chart<'a> {
    let datasets = state
        .datasets
        .iter()
        .zip([Color::Red, Color::Blue])
        .map(|((interface, points), color)| {
            Dataset::default()
                .name(interface.clone())
                .data(points)
                .graph_type(GraphType::Line)
                .style(color)
        })
        .collect();

    Chart::new(datasets)
        .block(Block::bordered().title(Line::from("Network Interface").bold().centered()))
        .x_axis(Axis::default().title("X Axis").bounds([0.0, 255.0]))
        .y_axis(Axis::default().title("Y Axis").bounds([0.0, 5000.0]))
        .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)))
}
