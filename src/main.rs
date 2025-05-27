use clap::Parser;
use std::path::PathBuf;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    // for Rust's panic debugging
    color_eyre::install()?;

    // Set up terminal
    let terminal = ratatui::init();
    run(terminal)?;
    ratatui::restore();

    Ok(())
}

fn run(mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

#[derive(Debug, Parser)]
struct CLIArgs {
    filepath: PathBuf,
}
