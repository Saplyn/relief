use derive_more::Display;
use serde::Deserialize;

pub const NEWEST_EDITION: usize = 0;
fn newest_edition() -> usize {
    NEWEST_EDITION
}

//~ Package Config

#[derive(Deserialize, Debug)]
pub struct PackageConfig {
    pub meta: PackageMeta,
    pub source: PackageSource,
    pub install: PackageInstall,
}

//~ Package Meta

#[derive(Deserialize, Debug)]
pub struct PackageMeta {
    #[serde(default = "newest_edition")]
    pub edition: usize,
    pub identifier: String,
}

//~ Package Source

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug, Default)]
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

#[derive(Deserialize, Debug)]
pub struct Github {
    pub owner: String,
    pub repo: String,
    pub asset: String,
    pub extract: Option<ExtractType>,
    pub version: GithubVersion,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug, Default, Display)]
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

#[derive(Deserialize, Debug)]
pub struct BinaryInstall {
    pub target: String,
    pub alias: Option<String>,
    pub version: BinaryVersion,
}

#[derive(Deserialize, Debug)]
pub struct BinaryVersion {
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

//~ Extract Type

#[derive(Deserialize, Debug, Display)]
#[serde(rename_all = "snake_case")]
pub enum ExtractType {
    Tar,
}
#[derive(Debug, Display)]
pub enum ExtractOption {
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
pub fn all_extract_option() -> Vec<ExtractOption> {
    vec![ExtractOption::None, ExtractOption::Tar]
}

//~ Utility Functions

pub fn default_version_regex() -> String {
    DEFAULT_VERSION_REGEX.to_string()
}
pub const DEFAULT_VERSION_REGEX: &str = "[0-9]+\\.[0-9]+\\.[0-9]+(-[a-zA-Z0-9]+)?";
