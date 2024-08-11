use std::time::Instant;

use clap::Parser;

use crate::{loading_bar::LoadingBar, on_key_press, basic_counters::BasicCounter};

#[derive(Parser)]
#[command(author, about, version)]
pub struct Cli {
    #[arg(short, long, help="A description to display with the counter")]
    description: Option<String>,
    #[arg(short, long, help="Whether you would like to display a loading bar")]
    loading_bar: bool,
    #[arg(short='y', long, help="Whether you would like to display a tally score")]
    tally: bool,
    #[arg(short, long, help="The total / value to count towards")]
    total: Option<u32>,
    #[arg(short, long, help="The amount to increment the loading bar by (defaults to \x1b[33m1\x1b[0m)")]
    increment: Option<u32>,
    #[arg(short='k', long="key", help="The key to indicate an increment (defaults to '\x1b[33mi\x1b[0m')")]
    increment_key: Option<char>,
}

impl Cli {
    pub fn run(self) {
        let increment = if let Some(increment) = self.increment { increment } else { 1 };
        let key = if let Some(key) = self.increment_key { key } else { 'i' };
        let description = if let Some(description) = self.description { description } else { "loading".into() };

        if self.loading_bar {
            if self.tally { println!("\x1b[31;1merror: \x1b[0mloading bar cannot have tally mark display"); std::process::exit(1) };
            let total = if let Some(total) = self.total { total } else { println!("\x1b[31;1merror: \x1b[0mloading bar must have a total / value to count towards"); std::process::exit(1) };

            let mut loading_bar = LoadingBar::new(total);
            let mut time = Instant::now();
            loading_bar.draw(&description);

            on_key_press(key, || {
                if loading_bar.update(time.elapsed().as_secs_f32(), increment)
                    .draw(&description) { return true };
                time = Instant::now();
                false
            });
        } else {
            let mut basic_counter = BasicCounter::new(self.total);
            let mut time = Instant::now();
            if self.tally { basic_counter.draw_tally(&description) } else { basic_counter.draw_data(&description) };

            on_key_press(key, || {
                if basic_counter.update(time.elapsed().as_secs_f32(), increment) {
                    if self.tally { basic_counter.draw_tally(&description) } else { basic_counter.draw_data(&description) };
                    return true
                }; if self.tally { basic_counter.draw_tally(&description) } else { basic_counter.draw_data(&description) };
                time = Instant::now();
                false
            });
        }
    }
}
