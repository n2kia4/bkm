#[macro_use]
extern crate clap;
extern crate dirs;
extern crate reqwest;
extern crate rusqlite;
extern crate select;

use clap::{App, AppSettings};
use select::document::Document;
use select::predicate::Name;
use std::process;

mod cmd;
mod database;

fn main() {
    let args = App::new("bkm")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Bookmark manager")
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand(cmd::print::make_subcommand())
        .subcommand(cmd::add::make_subcommand())
        .subcommand(cmd::delete::make_subcommand())
        .subcommand(cmd::update::make_subcommand())
        .get_matches();

    match args.subcommand() {
        ("print", Some(args)) => cmd::print::execute(args),
        ("add" , Some(args)) => cmd::add::execute(args),
        ("delete", Some(args)) => cmd::delete::execute(args),
        ("update", Some(args)) => cmd::update::execute(args),
        _ => process::exit(1),
    }
}

fn get_title_from_url(url: &str) -> String {
    let res = reqwest::get(url).expect("invalid URL");
    let doc = Document::from_read(res).unwrap();
    let title = doc.find(Name("title")).next().unwrap().text();
    title
}
