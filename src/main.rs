mod content;

use content::catalog::Catalog;
use content::media::Media;

fn main() {
    let book = Media::Book {
        title: String::from("Atomic Habits"),
        author: String::from("Bhakul das"),
    };
    let mut catalog = Catalog::new();

    catalog.add(book);
    match catalog.get_by_index(100) {
        Option::Some(value) => {
            println!("{:#?}", value);
        }
        Option::None => {
            println!("No value found");
        }
    }

    println!("{:#?}", catalog);
}
