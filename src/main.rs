use clap::Parser;
use cli::args::ReliefArgs;
use log::{info, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

mod cli;
mod config;
mod model;

fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Trace,
        ConfigBuilder::default()
            .add_filter_allow("relief".to_string())
            .build(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )?;
    info!("Log initialized!");

    let args = ReliefArgs::parse();
    info!("Parsed args: {:?}", args);

    match args {
        ReliefArgs::Pick(args) => cli::pick(args)?,
        ReliefArgs::Fetch(args) => cli::fetch(args)?,
        ReliefArgs::Drop(args) => cli::drop(args)?,
    }

    Ok(())
}
