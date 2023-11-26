use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    username: String,
    avatar_url: String,
    shortCode: String,
    pub(crate) embeds: Vec<Embeds>
}
impl Default for Post {
    fn default () -> Post {
        Post {
            username: String::new(),
            avatar_url: String::new(),
            shortCode: String::new(),
            embeds: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Embeds {
    pub title: String,
    pub color: String,
    pub url: String,
    pub description: String,
    pub timestamp: i64,
    pub author: Author,
    pub image: String,
    pub footer: Footer
}
impl Default for Embeds {
    fn default() -> Embeds {
        Embeds {
            title: String::new(),
            color: String::new(),
            url: String::new(),
            description: String::new(),
            timestamp: 0,
            author: Author::default(),
            image: String::new(),
            footer: Footer::default()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    name: String,
    icon_url: String
}
impl Default for Author {
    fn default() -> Author {
        Author {
            name: String::new(),
            icon_url: String::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Footer {
    icon_url: String,
    text: String
}
impl Default for Footer {
    fn default() -> Footer {
        Footer {
            icon_url: String::new(),
            text: String::new()
        }
    }
}