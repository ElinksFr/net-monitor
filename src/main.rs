use bpf::probs::LoadedProb;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use libbpf_rs::skel::{OpenSkel, SkelBuilder};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{error::Error, io::stdout};
use tui::{events::Event, render::draw_state, state::Model};

mod bandwidth_tracker;
mod bpf;
#[path = "bpf/.output/packet_size.skel.rs"]
mod packet_size;
mod tui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let opened_skel = packet_size::PacketSizeSkelBuilder::default().open()?;
    let mut skel = opened_skel.load()?;
    let _probs = LoadedProb::load_ebpf_monitoring_probs(&mut skel)?;

    let map_collection = skel.maps();
    let packet_stats = map_collection.packet_stats();

    let mut state_model = Model::init(packet_stats)?;

    loop {
        terminal.draw(|frame| draw_state(frame, &state_model))?;
        if crossterm::event::poll(std::time::Duration::from_millis(250))?
            && Event::try_from(crossterm::event::read()?) == Ok(Event::Quit)
        {
            break;
        }
        state_model = state_model.handel_event(&Event::Tick)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
