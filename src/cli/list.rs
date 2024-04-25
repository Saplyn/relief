//~ List

use log::info;
use thiserror::Error;

use super::args::ListArgs;

#[derive(Debug, Error)]
pub enum ListError {}

pub fn list(args: ListArgs) -> Result<(), ListError> {
    info!("List received args: {:?}", args);

    Ok(())
}
