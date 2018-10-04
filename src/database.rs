use rusqlite;
use rusqlite::Connection;
use dirs;
use std::fs;
use std::path::Path;

pub struct Bookmark {
    id: i64,
    title: String,
    url: String
}

impl Bookmark {
    pub fn new(id: i64, title: String, url: String) -> Self {
        Bookmark {
            id, title, url
        }
    }
}

pub struct Tag {
    pub id: i64,
    pub name: String
}

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

        conn.execute("CREATE TABLE IF NOT EXISTS bookmarks (
            id    INTEGER PRIMARY KEY,
            title    TEXT NOT NULL,
            url    TEXT NOT NULL UNIQUE
        )", &[]).unwrap();

        conn.execute("CREATE TABLE IF NOT EXISTS tags (
            id    INTEGER PRIMARY KEY,
            name    TEXT NOT NULL UNIQUE
        )", &[]).unwrap();

        conn.execute("CREATE TABLE IF NOT EXISTS bookmark_tag (
            bookmark_id    INTEGER NOT NULL,
            tag_id    INTEGER NOT NULL
        )", &[]).unwrap();

        DB {
            conn: conn
        }
    }

    pub fn get_all_bookmark(&self) -> Vec<Bookmark> {
        let query = "SELECT * FROM bookmarks";
        let mut stmt = self.conn.prepare(query).unwrap();

        let bookmark_iter = stmt.query_map(&[], |r| {
            Bookmark {
                id: r.get(0),
                title: r.get(1),
                url: r.get(2)
            }
        }).unwrap();

        let mut bookmarks: Vec<Bookmark> = Vec::new();
        for bookmark in bookmark_iter {
            bookmarks.push(bookmark.unwrap());
        }

        bookmarks
    }

    pub fn get_bookmark_by_id(&self, id: i64) -> Result<Bookmark, &str> {
        let query = "SELECT * FROM bookmarks WHERE id=?";

        match self.conn.query_row(query, &[&id], |r| {
            Bookmark {
                id: r.get(0),
                title: r.get(1),
                url: r.get(2)
            }
        }) {
            Ok(b) => Ok(b),
            Err(_) => Err("Error: Did not match index"),
        }
    }

    pub fn get_all_tag(&self) -> Vec<Tag> {
        let query = "SELECT * FROM tags";
        let mut stmt = self.conn.prepare(query).unwrap();

        let tag_iter = stmt.query_map(&[], |r| {
            Tag {
                id: r.get(0),
                name: r.get(1)
            }
        }).unwrap();

        let mut tags: Vec<Tag> = Vec::new();
        for tag in tag_iter {
            tags.push(tag.unwrap());
        }

        tags
    }

    pub fn add_bookmark(&self, bookmark: &Bookmark) -> Result<(), &str> {
        let query = "INSERT INTO bookmarks (title, url) VALUES ($1, $2)";

        match self.conn.execute(query, &[&bookmark.title, &bookmark.url]) {
            Ok(_) => Ok(()),
            Err(_) => Err("Error: URL already exists"),
        }
    }

    pub fn add_tag(&self, id: i64, tag: &str) {
        let select_query = "SELECT id FROM tags WHERE name=?";
        let insert_query = "INSERT INTO tags (name) VALUES ($1)";

        match self.conn.query_row(select_query, &[&tag], |r| { r.get(0) }) {
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

    pub fn delete(&self, table_name: &str, id: i64) {
        let query = format!("DELETE FROM {} WHERE id=?", table_name);
        self.conn.execute(query.as_str(), &[&id]).unwrap();
    }

    pub fn delete_bookmark_tag(&self, column: &str, id: i64) {
        let query = format!("DELETE FROM bookmark_tag WHERE {}=?", column);
        self.conn.execute(query.as_str(), &[&id]).unwrap();
    }

    pub fn clear(&self, table_name: &str) {
        let query = format!("DELETE FROM {}", table_name);
        self.conn.execute(query.as_str(), &[]).unwrap();
    }

    pub fn check_existence(&self, table_name: &str, id: i64) -> i64 {
        let query = format!("SELECT COUNT(*) FROM {} WHERE id=?", table_name);
        self.conn.query_row(query.as_str(), &[&id], |r| r.get(0)).unwrap()
    }

    pub fn get_record_count(&self, table_name: &str) -> i64 {
        let query = format!("SELECT count(*) from {}", table_name);
        self.conn.query_row(query.as_str(), &[], |r| r.get(0)).unwrap()
    }

    pub fn print_bookmark(&self, bookmark: Bookmark) {
        let space = "    ";
        println!("{} {}\n{}{}", bookmark.id, bookmark.title, space, bookmark.url);

        let tags = self.get_tags(&bookmark).unwrap();
        let t: Vec<&str> = tags.iter().map(AsRef::as_ref).collect();

        if t.join("") == "" {
            println!("");
        } else {
            println!("{}{}\n", space, t.join(", "));
        }
    }

    fn get_tags(&self, bookmark: &Bookmark) -> rusqlite::Result<Vec<String>> {
        let query = format!(
            "SELECT name FROM tags t LEFT JOIN bookmark_tag bt
            ON bt.tag_id=t.id WHERE bt.bookmark_id='{}'", bookmark.id
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
