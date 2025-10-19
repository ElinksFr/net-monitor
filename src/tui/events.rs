#[derive(PartialEq, Eq)]
pub enum Event {
    Tick,
    Quit,
}

fn is_shutdown_event(key_event: crossterm::event::KeyEvent) -> bool {
    if key_event.kind != crossterm::event::KeyEventKind::Press {
        false
    } else if matches!(key_event.code, crossterm::event::KeyCode::Char('q')) {
        true
    } else {
        matches!(key_event.code, crossterm::event::KeyCode::Char('c'))
            && matches!(key_event.modifiers, crossterm::event::KeyModifiers::CONTROL)
    }
}
#[derive(PartialEq, Eq)]
pub struct NoOp;
impl TryFrom<crossterm::event::Event> for Event {
    type Error = NoOp;

    fn try_from(value: crossterm::event::Event) -> Result<Self, Self::Error> {
        if let crossterm::event::Event::Key(key) = value {
            if is_shutdown_event(key) {
                return Ok(Event::Quit);
            }
        }
        Err(NoOp)
    }
}
