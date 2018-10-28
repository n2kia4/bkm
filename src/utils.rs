use reqwest;
use select::document::Document;
use select::predicate::Name;

pub fn get_title_from_url(url: &str) -> String {
    let res = reqwest::get(url).expect("invalid URL");
    let doc = Document::from_read(res).unwrap();

    doc.find(Name("title")).next().unwrap().text()
}
