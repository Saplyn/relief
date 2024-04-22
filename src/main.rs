use clap::Parser;
use cli::args::ReliefArgs;
use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

mod cli;
mod config;
mod model;

fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )?;
    info!("Log initialized!");

    let args = ReliefArgs::parse();
    info!("Parsed args: {:?}", args);

    match args {
        ReliefArgs::Pick(args) => cli::pick(args),
        ReliefArgs::Fetch(args) => cli::fetch(args),
        ReliefArgs::Drop(args) => cli::drop(args),
    }

    Ok(())
}
