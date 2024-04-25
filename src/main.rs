use std::fs;

use clap::Parser;
use cli::args::ReliefArgs;
use log::{info, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

use crate::config::app::AppConfig;

mod cli;
mod config;
mod model;
mod util;

fn main() -> anyhow::Result<()> {
    init_log()?;
    let app = init_app()?;
    let args = ReliefArgs::parse();
    info!("Parsed args: {:?}", args);

    match args {
        ReliefArgs::Pick(args) => cli::pick(&app, args)?,
        ReliefArgs::Fetch(args) => cli::fetch(args)?,
        ReliefArgs::Drop(args) => cli::drop(args)?,
        ReliefArgs::List(args) => cli::list(args)?,
        ReliefArgs::Edit(args) => cli::edit(args)?,
    };

    Ok(())
}

fn init_log() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Trace,
        ConfigBuilder::default()
            .add_filter_allow("relief".to_string())
            .build(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )?;
    info!("Log initialized!");

    Ok(())
}

fn init_app() -> anyhow::Result<AppConfig> {
    let app = AppConfig::new()?;
    info!("Running with config: {:?}", app);

    fs::create_dir_all(app.dirs.config_dir())?;
    info!("Ensured dir {:?} exist", app.dirs.config_dir());

    fs::create_dir_all(app.dirs.data_dir())?;
    info!("Ensured dir {:?} exist", app.dirs.data_dir());

    Ok(app)
}
