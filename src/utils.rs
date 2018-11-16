use reqwest;
use select::document::Document;
use select::predicate::Name;
use std::fs::File;
use std::io::BufReader;

use bookmark::Bookmark;

pub fn get_bookmarks_from_html(file_name: &str) -> Vec<Bookmark> {
    let file = File::open(file_name)
        .expect("File not found or cannot be opened");
    let doc = Document::from_read(BufReader::new(&file)).unwrap();
    let mut bookmarks: Vec<Bookmark> = Vec::new();

    for (i, a) in doc.find(Name("a")).enumerate() {
        let mut tags: Vec<String> = Vec::new();

        if let Some(f) = a.parent().unwrap().parent().unwrap()
            .parent().unwrap().find(Name("h3")).next() {
            tags.push(f.text());
        }

        if let Some(t) = a.attr("tags") {
            for tag in t.split(",") {
                tags.push(tag.to_string());
            }
        }

        tags.sort();
        tags.dedup();

        let bookmark = Bookmark::new(
            (i + 1) as i64,
            a.text(),
            a.attr("href").unwrap().to_string(),
            tags,
        );
        bookmarks.push(bookmark);
    }

    bookmarks
}

pub fn get_title_from_url(url: &str) -> String {
    let res = reqwest::get(url).expect("invalid URL");
    let doc = Document::from_read(res).unwrap();

    doc.find(Name("title")).next().unwrap().text()
}
