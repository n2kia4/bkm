use clap::ArgMatches;
use std::process;

use database::{DB, Bookmark};
use get_title_from_url;

pub fn execute(args: &ArgMatches) {
    let db = DB::open();

    let id = db.get_record_count("bookmarks") + 1;
    let url = args.value_of("URL").unwrap();
    let title = if let Some(title) = args.value_of("title") {
        title.to_string()
    } else {
        get_title_from_url(url)
    };

    let bookmark = Bookmark::new(
        id,
        title,
        url.to_string()
    );

    match db.add_bookmark(&bookmark) {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }

    if let Some(tags) = args.values_of("tag") {
        for tag in tags {
            db.add_tag(id, tag);
        }
    }

    db.print_bookmark(bookmark);
}
