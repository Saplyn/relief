use std::{fmt::Display, fs, io};

use inquire::{
    required,
    ui::{Attributes, Color, RenderConfig, Styled},
    Confirm, Select, Text,
};
use log::{debug, info};
use owo_colors::OwoColorize;
use thiserror::Error;

use crate::config::{
    all_extract_option, all_gh_ver_members, all_install_options_besides_noop, all_package_sources,
    BinaryInstall, BinaryVersion, Github, GithubVersion, PackageConfig, PackageInstall,
    PackageInstallOption, PackageMeta, PackageSource, PackageSourceTag, DEFAULT_VERSION_REGEX,
    NEWEST_EDITION,
};

use super::{args::PickArgs, default_render_config};

#[derive(Debug, Error)]
pub enum PickError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("{0}")]
    Inquire(#[from] inquire::InquireError),
}

pub fn pick(args: PickArgs) -> Result<(), PickError> {
    info!("Pick received args: {:?}", args);
    let config: PackageConfig = if let Some(path) = args.config_file {
        info!("Pick config file at: {:?}", path);
        let file = fs::read_to_string(path)?;
        toml::from_str(&file)?
    } else {
        pick_interactive(args)?
    };
    info!("Picker config: {:?}", config);

    debug!("package meta: \n{:#?}", config.meta);
    debug!("package source: \n{:#?}", config.source);
    debug!("package install: \n{:#?}", config.install);

    Ok(())
}

//~

//~ Interactive Pick Prompt

fn pick_interactive(args: PickArgs) -> Result<PackageConfig, PickError> {
    let cfg = default_render_config();
    let id = if let Some(id) = args.identifier {
        println!(
            "{} Package identifier: {}",
            '✓'.bright_green(),
            id.bright_cyan()
        );
        id
    } else {
        text_required("Package identifier:", cfg).prompt()?
    };

    let source = match select("Package source:", all_package_sources(), cfg).prompt()? {
        PackageSourceTag::Github => prompt_github(cfg)?,
    };

    let mut install = PackageInstall::default();
    let mut is_provided = false;
    let mut remain_install_opt = all_install_options_besides_noop();
    loop {
        let opt = select("Install option", remain_install_opt.clone(), cfg).prompt()?;
        remain_install_opt.retain(|item| item != &opt);
        match opt {
            PackageInstallOption::NoOp => break,
            PackageInstallOption::Binary => install.binary = Some(prompt_binary(cfg)?),
        }

        if !is_provided {
            is_provided = true;
            remain_install_opt.insert(0, PackageInstallOption::NoOp);
        }
    }

    Ok(PackageConfig {
        meta: PackageMeta {
            edition: NEWEST_EDITION,
            identifier: id,
        },
        source,
        install,
    })
}

fn prompt_github(cfg: RenderConfig) -> Result<PackageSource, PickError> {
    let cfg = cfg
        .with_prompt_prefix(Styled::new("GitHub").with_attr(Attributes::BOLD))
        .with_answered_prompt_prefix(Styled::new("GitHub").with_fg(Color::LightGreen));

    Ok(PackageSource::Github(Github {
        owner: text_required("Repo owner:", cfg).prompt()?,
        repo: text_required("Repo name:", cfg).prompt()?,
        asset: text_required("Release asset name:", cfg)
            .with_help_message("Use \"{version}\" to represent version numbers")
            .prompt()?,
        extract: select("Extract type:", all_extract_option(), cfg)
            .prompt()?
            .into(),
        version: if do_custom_version(cfg).prompt()? {
            GithubVersion {
                member: select("Release API JSON member:", all_gh_ver_members(), cfg).prompt()?,
                regex: version_regex("Content version regex matcher:", cfg).prompt()?,
            }
        } else {
            Default::default()
        },
    }))
}

fn prompt_binary(cfg: RenderConfig) -> Result<BinaryInstall, PickError> {
    let cfg = cfg
        .with_prompt_prefix(Styled::new("Binary").with_attr(Attributes::BOLD))
        .with_answered_prompt_prefix(Styled::new("Binary").with_fg(Color::LightGreen));

    Ok(BinaryInstall {
        target: text_required("Target name:", cfg).prompt()?,
        alias: Text::new("Alias binary target:")
            .with_help_message("leave it empty or press <ECS> for no alias")
            .with_render_config(cfg)
            .prompt_skippable()?
            .filter(|s| !s.is_empty()),
        version: if do_custom_version(cfg).prompt()? {
            BinaryVersion {
                arg: text_required("Version arg:", cfg).prompt()?,
                regex: version_regex("Output version regex matcher:", cfg).prompt()?,
            }
        } else {
            Default::default()
        },
    })
}

//~ Helper function

fn text_required<'m: 't, 'c: 't, 't>(message: &'m str, cfg: RenderConfig<'c>) -> Text<'t> {
    Text::new(message)
        .with_validator(required!("This field is required!"))
        .with_render_config(cfg)
}

fn select<'m: 's, 'c: 's, 's, T: Display>(
    message: &'m str,
    options: Vec<T>,
    cfg: RenderConfig<'c>,
) -> Select<'s, T> {
    Select::new(message, options).with_render_config(cfg)
}

fn do_custom_version<'c: 'y, 'y>(cfg: RenderConfig<'c>) -> Confirm<'y> {
    Confirm::new("Customize version config?")
        .with_render_config(cfg)
        .with_default(false)
}

fn version_regex<'m: 't, 'c: 't, 't>(message: &'m str, cfg: RenderConfig<'c>) -> Text<'t> {
    Text::new(message)
        .with_render_config(cfg)
        .with_default(DEFAULT_VERSION_REGEX)
}
