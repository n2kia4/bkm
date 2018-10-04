#[macro_use]
extern crate clap;
extern crate dirs;
extern crate reqwest;
extern crate rusqlite;
extern crate select;

use clap::{App, AppSettings, Arg, SubCommand};
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
        .subcommand(SubCommand::with_name("print")
                    .about("Print bookmark")
                    .arg(Arg::from_usage("<ID>... 'Print bookmarks matching the specified ids{n}\
                                         (Print all bookmarks if id is not specified)'").required(false))
                    .arg(Arg::from_usage("-T 'Print all tags'")))
        .subcommand(SubCommand::with_name("add")
                    .about("Add bookmark")
                    .arg(Arg::from_usage("<URL> 'Bookmark URL'"))
                    .arg(Arg::from_usage("-t --tag=[tag]... 'Add tags to bookmark'"))
                    .arg(Arg::from_usage("-i --title=[title] 'Decide bookmark title yourself'")))
        .subcommand(SubCommand::with_name("delete")
                    .about("Delete bookmark")
                    .arg(Arg::from_usage("<ID>... 'Delete bookmarks matching the specified ids{n}\
                                         (Delete all bookmarks if id is not specified)'").required(false))
                    .arg(Arg::from_usage("-T 'Delete all tags'"))
                    .arg(Arg::from_usage("-t --tag_id=[id]... 'Delete tags matching the specified ids'")))
        .subcommand(SubCommand::with_name("update")
                    .about("Update bookmark")
                    .arg(Arg::from_usage("<ID> 'Update bookmark matching the specified id'"))
                    .arg(Arg::from_usage("-u --url=[url] 'Update bookmark URL'"))
                    .arg(Arg::from_usage("-i --title=[title] 'Update bookmark title'"))
                    .arg(Arg::from_usage("-t --tag=[tag]... 'Update bookmark tags'"))
                    .after_help("If no option is specified, get title from url and update."))
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
