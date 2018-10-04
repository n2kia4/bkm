use clap::ArgMatches;

use database::{DB, Bookmark};
use get_title_from_url;

pub fn execute(args: &ArgMatches) {
    let db = DB::open();

    let id = value_t!(args, "ID", i64).unwrap();

    let url = if let Some(url) = args.value_of("url") {
        url.to_string()
    } else {
        db.get_url_by_id(id)
    };

    let title = if let Some(title) = args.value_of("title") {
        title.to_string()
    } else if ! args.is_present("title") &&
              ! args.is_present("url") &&
              ! args.is_present("tag") {
        get_title_from_url(&url)
    } else {
        db.get_title_by_id(id)
    };

    if let Some(tags) = args.values_of("tag") {
        db.delete_bookmark_tag("bookmark_id", id);
        for tag in tags {
            db.add_tag(id, tag);
        }
    }

    let bookmark = Bookmark::new(id, title, url);
    db.update(id, &bookmark);
    db.print_bookmark(bookmark);
}
