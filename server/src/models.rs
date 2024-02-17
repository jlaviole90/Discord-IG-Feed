use serde::{Deserialize, Serialize};

pub struct Post {
    pub username: String,
    pub embeds: Embeds,
}
impl Default for Post {
    fn default() -> Post {
        Post {
            username: String::new(),
            embeds: Embeds::default(),
        }
    }
}

pub struct Embeds {
    pub description: String,
    pub timestamp: i64,
    pub image: String,
}
impl Default for Embeds {
    fn default() -> Embeds {
        Embeds {
            description: String::new(),
            timestamp: 0,
            image: String::new(),
        }
    }
}

/*
 *  Instagram API data-model
 */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "edge_owner_to_timeline_media")]
    pub edge_owner_to_timeline_media: EdgeOwnerToTimelineMedia,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeOwnerToTimelineMedia {
    pub edges: Vec<Edge3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge3 {
    pub node: Node3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node3 {
    #[serde(rename = "display_url")]
    pub display_url: String,
    #[serde(rename = "is_video")]
    pub is_video: bool,
    #[serde(rename = "edge_media_to_caption")]
    pub edge_media_to_caption: EdgeMediaToCaption,
    #[serde(rename = "taken_at_timestamp")]
    pub taken_at_timestamp: i64,
    #[serde(rename = "video_url")]
    pub video_url: Option<String>,
    #[serde(rename = "pinned_for_users")]
    pub pinned_for_users: Vec<PinnedForUser>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeMediaToCaption {
    pub edges: Vec<Edge4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge4 {
    pub node: Node4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node4 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PinnedForUser {
    pub id: String,
    #[serde(rename = "is_verified")]
    pub is_verified: bool,
    #[serde(rename = "profile_pic_url")]
    pub profile_pic_url: String,
    pub username: String,
}
