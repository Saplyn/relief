use inquire::ui::{Attributes, Color, ErrorMessageRenderConfig, RenderConfig, Styled};
use log::info;
use thiserror::Error;

use self::args::{DropArgs, EditArgs, FetchArgs, ListArgs};

pub mod args;
mod pick;

#[allow(unused_imports)]
pub use pick::{pick, PickError};

//~ Fetch

#[derive(Debug, Error)]
pub enum FetchError {}

pub fn fetch(args: FetchArgs) -> Result<(), FetchError> {
    info!("Fetch received args: {:?}", args);

    Ok(())
}

//~ Drop

#[derive(Debug, Error)]
pub enum ListError {}

pub fn list(args: ListArgs) -> Result<(), DropError> {
    info!("List received args: {:?}", args);

    Ok(())
}

//~ Edit

#[derive(Debug, Error)]
pub enum EditError {}

pub fn edit(args: EditArgs) -> Result<(), DropError> {
    info!("Edit received args: {:?}", args);

    Ok(())
}

//~ Drop

#[derive(Debug, Error)]
pub enum DropError {}

pub fn drop(args: DropArgs) -> Result<(), DropError> {
    info!("Drop received args: {:?}", args);

    Ok(())
}

//~ Helper function

fn default_render_config<'a>() -> RenderConfig<'a> {
    RenderConfig::default_colored()
        .with_prompt_prefix(Styled::new("?").with_attr(Attributes::BOLD))
        .with_answered_prompt_prefix(Styled::new("✓").with_fg(Color::LightGreen))
        .with_highlighted_option_prefix(Styled::new("▸").with_fg(Color::LightCyan))
        .with_error_message(
            ErrorMessageRenderConfig::default_colored()
                .with_prefix(Styled::new("x").with_fg(Color::LightRed)),
        )
}
