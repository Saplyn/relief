use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ReleaseLatest {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: usize,
    pub author: User,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<Asset>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: String,
    pub reactions: Reaction,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub id: usize,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub site_admin: bool,
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub url: String,
    pub id: usize,
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub uploader: String,
    pub content_type: String,
    pub state: String,
    pub size: usize,
    pub download_count: usize,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Reaction {
    pub url: String,
    pub total_count: usize,
    #[serde(rename = "+1")]
    pub plus_one: usize,
    #[serde(rename = "-1")]
    pub minus_one: usize,
    pub laugh: usize,
    pub hooray: usize,
    pub confused: usize,
    pub heart: usize,
    pub rocket: usize,
    pub eyes: usize,
}
