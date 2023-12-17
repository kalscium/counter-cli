use std::io::{self, Write};

use crossterm::cursor;

use crate::loading_bar::LoadingBar;

pub struct BasicCounter {
    total: Option<u32>,
    current: u32,
    avg: Option<f32>,
    buffer: String,
}

impl BasicCounter {
    pub fn new(total: Option<u32>) -> Self {
        Self {
            total,
            current: 0,
            avg: None,
            buffer: " ".repeat(crossterm::terminal::size().unwrap_or((2, 0)).0 as usize -2),
        }
    }
}


impl BasicCounter {
    /// Updates the counter by the given amount and returns whether the counter is full\
    pub fn update(&mut self, s_elapsed: f32, increment: u32) -> bool {
        let clamped = increment.clamp(0, self.total.unwrap_or(u32::MAX) - self.current);
        self.current += clamped;
        self.avg = match self.avg {
            Some(avg) => Some((avg + clamped as f32 / s_elapsed) / 2.0),
            None => Some(clamped as f32),
        }; self.current == self.total.unwrap_or(u32::MAX)
    }

    /// Draws the counter as raw data
    pub fn draw_data(&self, description: &str) {
        let mut stdout = io::stdout();

        let percent = self.total.map(|total| self.current as f32 * 100.0 / total as f32);
        let eta = self.total.map(|total| (total - self.current) as f32 / self.avg.unwrap_or(1.0));

        // wipe the lines that were previously drawn
        print!("{}{}{}{}", cursor::MoveToColumn(0), self.buffer, cursor::MoveToColumn(0), cursor::Hide);

        write!(
            stdout,
            "\x1b[36;1minfo: \x1b[0m{description}... \x1b[34m( \x1b[33m{}{}{}\x1b[34m )",
            match self.total {
                Some(total) => format!("{}\x1b[34m/\x1b[33m{total}", self.current),
                None => self.current.to_string(),
            },
            match self.avg {
                Some(avg) => format!("\x1b[34m, \x1b[33m{avg:.2}/s"),            
                None => String::new(),
            },
            match self.total {
                Some(_) => format!("\x1b[34m, \x1b[33m{:.2}%{}", percent.unwrap(), match eta {
                    Some(eta) => format!("\x1b[34m, \x1b[36;1meta: \x1b[0m\x1b[33m{:.2}", LoadingBar::right_time_unit(eta)),
                    None => String::new(),
                }),
                None => String::new(),
            },
        ).unwrap();

        stdout.flush().unwrap();
    }

    /// Draws the data as tally marks
    pub fn draw_tally(&self, description: &str) {
        self.draw_data(description);

        // wipe the lines that were previously drawn
        print!("\n{}{}{}", cursor::MoveToColumn(0), self.buffer, cursor::MoveToColumn(0));

        print!(
            "    > \x1b[33;1m{}{}{}\x1b[0m",
            "𝍸 ".repeat(self.current as usize / 5),
            "𝍷".repeat(self.current as usize % 5),
            cursor::MoveUp(1),
        ); io::stdout().flush().unwrap();
    }
}