use derive_more::Display;
use serde::{Deserialize, Serialize};

pub const NEWEST_EDITION: usize = 0;
fn newest_edition() -> usize {
    NEWEST_EDITION
}

//~ Package Config

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageConfig {
    pub meta: PackageMeta,
    pub source: PackageSource,
    pub install: PackageInstall,
}

//~ Package Meta

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageMeta {
    #[serde(default = "newest_edition")]
    pub edition: usize,
    pub identifier: String,
}

//~ Package Source

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PackageSource {
    Github(Github),
}
#[derive(Debug, Display)]
pub enum PackageSourceTag {
    #[display(fmt = "GitHub Release")]
    Github,
}
pub fn all_package_sources() -> Vec<PackageSourceTag> {
    vec![PackageSourceTag::Github]
}

//~ Package Install

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PackageInstall {
    pub binary: Option<BinaryInstall>,
}
#[derive(Debug, Display, PartialEq, Eq, Clone, Copy)]
pub enum PackageInstallOption {
    #[display(fmt = "No other install action")]
    NoOp,
    #[display(fmt = "Install as binary executable")]
    Binary,
}
pub fn all_install_options_besides_noop() -> Vec<PackageInstallOption> {
    vec![PackageInstallOption::Binary]
}

//~ Github Release Config

#[derive(Serialize, Deserialize, Debug)]
pub struct Github {
    pub owner: String,
    pub repo: String,
    pub asset: String,
    pub extract: Option<ExtractType>,
    #[serde(default = "GithubVersion::default")]
    pub version: GithubVersion,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubVersion {
    #[serde(default = "default_gh_ver_member")]
    pub member: GhVerMember,
    #[serde(default = "default_version_regex")]
    pub regex: String,
}
impl Default for GithubVersion {
    fn default() -> Self {
        Self {
            regex: default_version_regex(),
            member: GhVerMember::default(),
        }
    }
}

//~ Github API JSON Member

#[derive(Serialize, Deserialize, Debug, Default, Display)]
#[serde(rename_all = "snake_case")]
pub enum GhVerMember {
    #[default]
    #[display(fmt = "`.tag_name`")]
    TagName,
    #[display(fmt = "`.name`")]
    Name,
}
pub fn all_gh_ver_members() -> Vec<GhVerMember> {
    vec![GhVerMember::TagName, GhVerMember::Name]
}
fn default_gh_ver_member() -> GhVerMember {
    GhVerMember::TagName
}

//~ Binary Install

#[derive(Serialize, Deserialize, Debug)]
pub struct BinaryInstall {
    pub target: String,
    pub rename: Option<String>,
    #[serde(default = "BinaryVersion::default")]
    pub version: BinaryVersion,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BinaryVersion {
    #[serde(default = "default_version_arg")]
    pub arg: String,
    #[serde(default = "default_version_regex")]
    pub regex: String,
}
impl Default for BinaryVersion {
    fn default() -> Self {
        Self {
            arg: String::from("--version"),
            regex: default_version_regex(),
        }
    }
}
fn default_version_arg() -> String {
    "--version".to_string()
}

//~ Extract Type

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ExtractType {
    Tar,
}
#[derive(Debug, Display, Clone, Copy)]
pub enum ExtractOption {
    #[display(fmt = "Don't extract")]
    None,
    Tar,
}
impl From<ExtractOption> for Option<ExtractType> {
    fn from(value: ExtractOption) -> Self {
        match value {
            ExtractOption::None => None,
            ExtractOption::Tar => Some(ExtractType::Tar),
        }
    }
}
impl From<Option<ExtractType>> for ExtractOption {
    fn from(value: Option<ExtractType>) -> Self {
        match value {
            None => ExtractOption::None,
            Some(ExtractType::Tar) => ExtractOption::Tar,
        }
    }
}
pub fn all_extract_option() -> Vec<ExtractOption> {
    vec![ExtractOption::None, ExtractOption::Tar]
}

//~ Utility Functions

fn default_version_regex() -> String {
    DEFAULT_VERSION_REGEX.to_string()
}
pub const DEFAULT_VERSION_REGEX: &str = "[0-9]+\\.[0-9]+\\.[0-9]+(-[a-zA-Z0-9]+)?";
