use rusqlite;
use rusqlite::Connection;
use dirs;
use std::fs;
use std::path::Path;

use bookmark::Bookmark;

pub struct DB {
    conn: Connection
}

impl DB {
    pub fn open() -> DB {
        let home_dir = dirs::home_dir().unwrap();
        let bkm_dir = format!("{}/.bkm", home_dir.display());
        let path = Path::new(&bkm_dir);
        fs::create_dir_all(&path).unwrap();

        let full_path = format!("{}/bookmarks.db", bkm_dir);
        let conn = Connection::open(full_path).unwrap();
        let db = DB { conn: conn };

        db.init();
        db
    }

    fn init(&self) {
        self.conn.execute("CREATE TABLE IF NOT EXISTS bookmarks (
            id    INTEGER PRIMARY KEY,
            title    TEXT NOT NULL,
            url    TEXT NOT NULL UNIQUE
        )", &[]).unwrap();

        self.conn.execute("CREATE TABLE IF NOT EXISTS tags (
            id    INTEGER PRIMARY KEY,
            name    TEXT NOT NULL UNIQUE
        )", &[]).unwrap();

        self.conn.execute("CREATE TABLE IF NOT EXISTS bookmark_tag (
            bookmark_id    INTEGER NOT NULL,
            tag_id    INTEGER NOT NULL
        )", &[]).unwrap();
    }

    pub fn get_all_bookmark(&self) -> Vec<Bookmark> {
        let query = "SELECT * FROM bookmarks";
        self.vectorize_bookmarks(query)
    }

    pub fn get_bookmark_by_id(&self, id: i64) -> Result<Bookmark, &str> {
        let query = "SELECT * FROM bookmarks WHERE id=?";

        match self.conn.query_row(query, &[&id], |r| {
            Bookmark {
                id: r.get(0),
                title: r.get(1),
                url: r.get(2),
                tags: self.get_tags(r.get(0)).unwrap()
            }
        }) {
            Ok(b) => Ok(b),
            Err(_) => Err("Error: Did not match index"),
        }
    }

    pub fn get_all_tag(&self) -> Vec<String> {
        let query = "SELECT * FROM tags";
        let mut stmt = self.conn.prepare(query).unwrap();

        let tag_iter = stmt.query_map(&[], |r| r.get(1)).unwrap();

        let mut tags: Vec<String> = Vec::new();
        for tag in tag_iter {
            tags.push(tag.unwrap());
        }

        tags
    }

    pub fn add_bookmark(&self, title: &String, url: &String) -> Result<(), &str> {
        let query = "INSERT INTO bookmarks (title, url) VALUES ($1, $2)";

        match self.conn.execute(query, &[title, url]) {
            Ok(_) => Ok(()),
            Err(_) => Err("Error: URL already exists"),
        }
    }

    pub fn add_tag(&self, id: i64, tag: &str) {
        let select_query = "SELECT id FROM tags WHERE name=?";
        let insert_query = "INSERT INTO tags (name) VALUES ($1)";

        match self.conn.query_row(select_query, &[&tag], |r| r.get(0)) {
            Ok(tag_id) => self.add_bookmark_tag(id, tag_id),
            Err(_) => {
                self.conn.execute(insert_query, &[&tag]).unwrap();
                let tag_id = self.conn.last_insert_rowid();
                self.add_bookmark_tag(id, tag_id);
            }
        }
    }

    pub fn add_bookmark_tag(&self, bookmark_id: i64, tag_id: i64) {
        let query = "INSERT INTO bookmark_tag (bookmark_id, tag_id) VALUES ($1, $2)";
        self.conn.execute(query, &[&bookmark_id, &tag_id]).unwrap();
    }

    pub fn delete_bookmark(&self, id: i64) {
        self.delete_bookmark_tag_by_id(id);

        let query = "DELETE FROM bookmarks WHERE id=?";
        self.conn.execute(query, &[&id]).unwrap();
    }

    pub fn delete_bookmark_tag_by_id(&self, id: i64) {
        let query = "DELETE FROM bookmark_tag WHERE bookmark_id=?";
        self.conn.execute(query, &[&id]).unwrap();
    }

    pub fn delete_tag(&self, name: &str) {
        self.delete_bookmark_tag_by_name(name);

        let query = "DELETE FROM tags WHERE name=?";
        self.conn.execute(query, &[&name]).unwrap();
    }

    fn delete_bookmark_tag_by_name(&self, name: &str) {
        let query = "DELETE FROM bookmark_tag WHERE tag_id IN (SELECT id FROM tags WHERE name=?)";
        self.conn.execute(query, &[&name]).unwrap();
    }

    pub fn clear(&self, table_name: &str) {
        let query = format!("DELETE FROM {}", table_name);
        self.conn.execute(query.as_str(), &[]).unwrap();
    }

    pub fn check_existence_bookmark(&self, id: i64) -> i64 {
        let query = "SELECT COUNT(*) FROM bookmarks WHERE id=?";
        self.conn.query_row(query, &[&id], |r| r.get(0)).unwrap()
    }

    pub fn check_existence_tag(&self, name: &str) -> i64 {
        let query = "SELECT COUNT(*) FROM tags WHERE name=?";
        self.conn.query_row(query, &[&name], |r| r.get(0)).unwrap()
    }

    pub fn update_bookmark(&self, id: i64, title: &String, url: &String) {
        let query = "Update bookmarks SET title = $1, url = $2 WHERE id=?";
        self.conn.execute(query, &[title, url, &id])
            .expect("Failed to update");
    }

    pub fn search(&self, keywords: Vec<&str>) -> Vec<Bookmark> {
        let query = format!(
            "SELECT * FROM bookmarks WHERE (title || url) LIKE \"%{}%\"",
            keywords.join("%")
        );

        self.vectorize_bookmarks(query.as_str())
    }

    pub fn search_by_tag(&self, keywords: Vec<&str>) -> Vec<Bookmark> {
        let query = format!(
            "select b.id, b.title, b.url from bookmark_tag bt
            inner join bookmarks b on b.id = bt.bookmark_id
            inner join tags t on t.id = bt.tag_id
            where t.name like \"%{}%\"", keywords.join("%")
        );

        self.vectorize_bookmarks(query.as_str())
    }

    fn vectorize_bookmarks(&self, query: &str) -> Vec<Bookmark> {
        let mut stmt = self.conn.prepare(query).unwrap();

        let bookmark_iter = stmt.query_map(&[], |r| {
            Bookmark {
                id: r.get(0),
                title: r.get(1),
                url: r.get(2),
                tags: self.get_tags(r.get(0)).unwrap()
            }
        }).unwrap();

        let mut bookmarks: Vec<Bookmark> = Vec::new();
        for bookmark in bookmark_iter {
            bookmarks.push(bookmark.unwrap());
        }

        bookmarks
    }

    pub fn get_url_by_id(&self, id: i64) -> String {
        let query = "SELECT url FROM bookmarks WHERE id=?";
        let url: String = self.conn.query_row(
            query, &[&id], |r| r.get(0)).expect("no match index");

        url
    }

    pub fn get_title_by_id(&self, id: i64) -> String {
        let query = "SELECT title FROM bookmarks WHERE id=?";
        let title = self.conn.query_row(
            query, &[&id], |r| r.get(0)).expect("no match index");

        title
    }

    pub fn get_record_count(&self, table_name: &str) -> i64 {
        let query = format!("SELECT count(*) from {}", table_name);
        self.conn.query_row(query.as_str(), &[], |r| r.get(0)).unwrap()
    }

    pub fn get_max_bookmark_id(&self) -> i64 {
        let query = "SELECT MAX(id) FROM bookmarks";
        self.conn.query_row(query, &[], |r| r.get(0)).unwrap()
    }

    pub fn get_tags(&self, bookmark_id: i64) -> rusqlite::Result<Vec<String>> {
        let query = format!(
            "SELECT name FROM tags t LEFT JOIN bookmark_tag bt
            ON bt.tag_id=t.id WHERE bt.bookmark_id='{}'", bookmark_id
        );
        let mut stmt = self.conn.prepare(query.as_str()).unwrap();
        let tag_iter = stmt.query_map(&[], |r| r.get(0))?;

        let mut tags: Vec<String> = Vec::new();
        for tag in tag_iter {
            tags.push(tag.unwrap());
        }

        Ok(tags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Bookmark> {
        let data: Vec<Bookmark> = vec![
            (Bookmark {
                id: 1,
                title: "GitHub".to_string(),
                url: "https://github.com".to_string(),
                tags: vec!["Git".to_string(), "Hosting service".to_string()],
            }),
            (Bookmark {
                id: 2,
                title: "Google".to_string(),
                url: "https://google.com".to_string(),
                tags: vec!["Search".to_string()],
            }),
            (Bookmark {
                id: 3,
                title: "Example Domain".to_string(),
                url: "https://example.com".to_string(),
                tags: vec!["".to_string()],
            }),
        ];

        data
    }

    fn open() -> DB {
        let conn = Connection::open_in_memory().unwrap();
        let db = DB { conn: conn};

        db.init();
        db
    }

    #[test]
    fn test_get_bookmark_by_id() {
        let db = open();

        for bookmark in test_data() {
            db.add_bookmark(&bookmark.title, &bookmark.url).unwrap();

            let b = db.get_bookmark_by_id(bookmark.id).unwrap();
            assert_eq!((bookmark.id, bookmark.title, bookmark.url),
                       (b.id, b.title, b.url));
        }
    }

    #[test]
    fn test_add_bookmark() {
        let db = open();

        for bookmark in test_data() {
            db.add_bookmark(&bookmark.title, &bookmark.url).unwrap();
            assert!(db.get_bookmark_by_id(bookmark.id).is_ok());
        }
    }

    #[test]
    fn test_delete_bookmark() {
        let db = open();
        let bookmark = &test_data()[0];

        db.add_bookmark(&bookmark.title, &bookmark.url).unwrap();
        db.delete_bookmark(bookmark.id);
        assert!(db.get_bookmark_by_id(bookmark.id).is_err());
    }

    #[test]
    fn test_update_bookmark() {
        let db = open();
        let old_bookmark = &test_data()[0];
        let new_bookmark = &test_data()[1];

        db.add_bookmark(&old_bookmark.title, &old_bookmark.url).unwrap();
        db.update_bookmark(old_bookmark.id, &new_bookmark.title, &new_bookmark.url);
        assert!(db.get_bookmark_by_id(old_bookmark.id).is_ok());
    }

    #[test]
    fn test_search() {
        let db = open();

        for bookmark in test_data() {
            db.add_bookmark(&bookmark.title, &bookmark.url).unwrap();

            let t_search = &db.search(vec![&bookmark.title])[0];
            let u_search = &db.search(vec![&bookmark.url])[0];

            assert_eq!((&bookmark.id, &bookmark.title, &bookmark.url),
                       (&t_search.id, &t_search.title, &t_search.url));
            assert_eq!((&bookmark.id, &bookmark.title, &bookmark.url),
                       (&u_search.id, &u_search.title, &u_search.url));
        }
    }
}
