use serde::Deserialize;

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
    pub edition: usize,
    pub identifier: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PackageSource {
    Github(GithubRelease),
}

#[derive(Deserialize, Debug)]
pub struct PackageInstall {
    pub binary: Option<BinaryInstall>,
}

//~ Github Release

#[derive(Deserialize, Debug)]
pub struct GithubRelease {
    pub owner: String,
    pub repo: String,
    pub asset: String,
    pub extract: Option<ExtractType>,
}

#[derive(Deserialize, Debug)]
pub struct GithubReleaseVersion {
    #[serde(default = "default_gh_ver_member")]
    pub member: GhVerMember,
    #[serde(default = "default_version_regex")]
    pub regex: String,
}

//~ Binary Install

#[derive(Deserialize, Debug)]
pub struct BinaryInstall {
    pub target: String,
    pub alias: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BinaryVersion {
    pub arg: String,
    #[serde(default = "default_version_regex")]
    pub regex: String,
}

//~ Utility Types

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExtractType {
    Tar,
}

#[derive(Deserialize, Debug)]
pub enum GhVerMember {
    TagName,
}

//~ Utility Functions

fn default_gh_ver_member() -> GhVerMember {
    GhVerMember::TagName
}

fn default_version_regex() -> String {
    "[0-9]+\\.[0-9]+\\.[0-9]+(-[a-zA-Z0-9]+)?".to_string()
}
