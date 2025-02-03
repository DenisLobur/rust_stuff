#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("{} by {}", title, author),
            Media::Movie { title, director } => format!("{} by {}", title, director),
            Media::Audiobook { title } => format!("{} by {}", title, "unknown"),
            Media::Podcast(episode) => format!("Episode {}", episode),
            Media::Placeholder => String::from("Placeholder"),
        }
    }

    fn description2(&self) -> String {
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

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn print(&self) {
        for item in &self.items {
            println!("{}", item.description());
        }
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        self.items.get(index as usize)
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook {
        title: "1984".to_string(),
    };

    let movie = Media::Movie {
        title: "The Matrix".to_string(),
        director: "The Wachowskis".to_string(),
    };

    let podcast = Media::Podcast(10);

    let placeholder = Media::Placeholder;

    //println!("{}", media.description2());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog);

    let item =  catalog.get_by_index(10);
    let placeholder =  Media::Placeholder;

    // println!("{:#?}", item.unwrap());
    // println!("{:#?}", item.expect("No item found"));
    println!("{:#?}", item.unwrap_or(&placeholder));
}
