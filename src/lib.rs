use crossterm::{cursor, ExecutableCommand};

pub mod loading_bar;
pub mod cli;
pub mod basic_counters;

pub fn on_key_press(expected_key: char, mut f: impl FnMut() -> bool) {
    use crossterm::{terminal, event::{self, Event, KeyCode}};
    terminal::enable_raw_mode().unwrap();
    loop {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char(x) if expected_key == x => if f() { break },
                KeyCode::Char('x') | KeyCode::Esc | KeyCode::Char('c') | KeyCode::Char('q') => break,
                _ => (),
            }
        }
    } terminal::disable_raw_mode().unwrap();
    std::io::stdout().execute(cursor::Show).unwrap();
}