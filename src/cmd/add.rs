use clap::{App, ArgMatches, SubCommand};
use std::process;

use database::{DB, Bookmark};
use get_title_from_url;

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add")
        .about("Add bookmark")
        .arg_from_usage("<URL> 'Bookmark URL'")
        .arg_from_usage("-t --tag=[tag]... 'Add tags to bookmark'")
        .arg_from_usage("-i --title=[title] 'Decide bookmark title yourself'")
}

pub fn execute(args: &ArgMatches) {
    let db = DB::open();

    let url = args.value_of("URL").unwrap();

    let title = if let Some(title) = args.value_of("title") {
        title.to_string()
    } else {
        get_title_from_url(url)
    };

    match db.add_bookmark(&title, &url.to_string()) {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }

    let id = db.get_max_bookmark_id();

    if let Some(tags) = args.values_of("tag") {
        for tag in tags {
            db.add_tag(id, tag);
        }
    }

    let bookmark = Bookmark::new(
        id, title, url.to_string()
    );

    db.print_bookmark(bookmark);
}
