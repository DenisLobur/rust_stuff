#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("{} by {}", title, author),
            Media::Movie { title, director } => format!("{} by {}", title, director),
            Media::Audiobook { title } => format!("{} by {}", title, "unknown"),
            Media::Podcast(episode) => format!("Episode {}", episode),
            Media::Placeholder => String::from("Placeholder"),
        }
    }

    pub fn description2(&self) -> String {
        if let Media::Book { title, author } = self {
            format!("Book: {} by {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Movie: {} by {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("Audiobook: {} by {}", title, "unknown")
        } else if let Media::Podcast(episode) = self {
            format!("Podcast: Episode {}", episode)
        } else {
            String::from("Unknown media type")
        }
    }
}