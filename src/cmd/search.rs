use clap::{App, ArgMatches, SubCommand};
use std::process;

use database::DB;

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("search")
        .about("Search bookmark")
        .arg_from_usage("<KEYWORD>... 'Search bookmarks with keywords in title or URL'")
        .arg_from_usage("-t --tag 'Search bookmark with tag'")
}

pub fn execute(args: &ArgMatches) {
    let db = DB::open();

    let keywords: Vec<&str> = args.values_of("KEYWORD")
        .unwrap().collect();

    let bookmarks = if args.is_present("tag") {
        db.search_by_tag(keywords)
    } else {
        db.search(keywords)
    };

    if bookmarks.len() == 0 {
        println!("Error: No matching any bookmark");
        process::exit(1);
    }

    for bookmark in bookmarks {
        bookmark.print();
    }
}
