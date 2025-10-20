use bpf::probs::LoadedProb;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use libbpf_rs::skel::{OpenSkel, SkelBuilder};
use ratatui::{
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    error::Error,
    io::{self, stdout},
    mem::MaybeUninit,
    time::Duration,
};
use tui::{events::Event, render::draw_state, state::Model};

mod bandwidth_tracker;
mod bpf;
#[path = "bpf/.output/packet_size.skel.rs"]
mod packet_size;
mod tui;

fn main() -> Result<(), Box<dyn Error>> {
    init_panic_hook();
    let mut open_object = MaybeUninit::uninit();
    let opened_skel = packet_size::PacketSizeSkelBuilder::default().open(&mut open_object)?;
    let mut skel = opened_skel.load()?;
    let _probs = LoadedProb::load_ebpf_monitoring_probs(&mut skel)?;

    let map_collection = skel.maps;
    let packet_stats = map_collection.packet_stats;

    let refresh_rate = Duration::from_millis(250);
    let mut state_model = Model::init(&packet_stats, refresh_rate)?;
    let mut terminal = init_tui()?;

    loop {
        terminal.draw(|frame| draw_state(frame, &state_model))?;
        if crossterm::event::poll(state_model.refresh_rate)?
            && Event::try_from(crossterm::event::read()?) == Ok(Event::Quit)
        {
            break;
        }
        state_model = state_model.handel_event(&Event::Tick)?;
    }

    restore_tui()?;
    Ok(())
}

pub fn init_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // intentionally ignore errors here since we're already in a panic
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}

pub fn init_tui() -> io::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore_tui() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
