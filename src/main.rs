#[macro_use]
extern crate clap;
extern crate dirs;
extern crate reqwest;
extern crate rusqlite;
extern crate select;
extern crate webbrowser;

use clap::{App, AppSettings};
use std::process;

mod cmd;
mod bookmark;
mod database;
mod utils;

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
        .subcommand(cmd::open::make_subcommand())
        .subcommand(cmd::search::make_subcommand())
        .get_matches();

    match args.subcommand() {
        ("print", Some(args)) => cmd::print::execute(args),
        ("add" , Some(args)) => cmd::add::execute(args),
        ("delete", Some(args)) => cmd::delete::execute(args),
        ("update", Some(args)) => cmd::update::execute(args),
        ("open", Some(args)) => cmd::open::execute(args),
        ("search", Some(args)) => cmd::search::execute(args),
        _ => process::exit(1),
    }
}
