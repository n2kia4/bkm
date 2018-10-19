pub struct Bookmark {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub tags: Vec<String>
}

impl Bookmark {
    pub fn new(id: i64, title: String, url: String, tags: Vec<String>) -> Self {
        Bookmark { id, title, url, tags }
    }

    pub fn print(&self) {
        let space = "    ";
        let bookmark = format!("{} {}\n{}{}\n", self.id, self.title, space, self.url);
        if self.tags.is_empty() {
            println!("{}", bookmark);
        } else {
            println!("{}{}{}\n", bookmark, space, self.tags.join(", "));
        }
    }
}
