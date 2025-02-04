mod content;

use content::media::Media;
use content::catalog::Catalog;

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
