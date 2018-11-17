use reqwest;
use select::document::Document;
use select::predicate::Name;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use bookmark::Bookmark;

pub fn get_bookmarks_from_html(path: PathBuf) -> Vec<Bookmark> {
    let file = File::open(path)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::Path;

    #[test]
    fn test_get_bookmarks_from_html() {
        let testdata: Vec<Bookmark> = vec![
            (Bookmark {
                id: 1,
                title: "GitHub".to_string(),
                url: "https://github.com".to_string(),
                tags: vec!["Git".to_string(), "Hosting Service".to_string(), "test".to_string()],
            }),
            (Bookmark {
                id: 2,
                title: "Google".to_string(),
                url: "https://google.com".to_string(),
                tags: vec!["test".to_string()],
            }),
            (Bookmark {
                id: 3,
                title: "Example Domain".to_string(),
                url: "https://example.com".to_string(),
                tags: vec!["Test".to_string()],
            }),
        ];

        let mut path = env::current_dir().unwrap();
        path.push(Path::new(file!()).parent().unwrap());
        path.push("testdata");
        path.push("bookmarks.html");

        let bookmarks: Vec<Bookmark> = get_bookmarks_from_html(path);

        for (test_bookmark, bookmark) in testdata.iter().zip(bookmarks.iter()) {
            assert_eq!((&test_bookmark.title, &test_bookmark.url, &test_bookmark.tags),
                       (&bookmark.title, &bookmark.url, &bookmark.tags));
        }
    }
}
