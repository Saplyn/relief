use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub enum ReliefArgs {
    Pick(PickArgs),
    Fetch(FetchArgs),
    Drop(DropArgs),
    List(ListArgs),
    Edit(EditArgs),
}

#[derive(Parser, Debug)]
pub struct PickArgs {
    #[arg(group = "pick_method")]
    pub identifier: Option<String>,
    #[arg(short = 'f', long = "file", group = "pick_method")]
    pub config_file: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct FetchArgs {
    pub identifier: String,
}

#[derive(Parser, Debug)]
pub struct DropArgs {
    pub identifier: String,
}

#[derive(Parser, Debug)]
pub struct ListArgs {}

#[derive(Parser, Debug)]
pub struct EditArgs {
    pub identifier: String,
}
