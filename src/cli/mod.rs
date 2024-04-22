use log::info;

use self::args::{DropArgs, FetchArgs, PickArgs};

pub mod args;

pub fn pick(args: PickArgs) {
    info!("`pick` received args: {:?}", args);

    todo!()
}

pub fn fetch(args: FetchArgs) {
    info!("`fetch` received args: {:?}", args);
}

pub fn drop(args: DropArgs) {
    info!("`drop` received args: {:?}", args);
}
