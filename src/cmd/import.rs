use clap::{App, ArgMatches, SubCommand};
use std::path::Path;

use database::DB;
use utils::get_bookmarks_from_html;

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("import")
        .about("Import bookmark")
        .arg_from_usage("<FILE> 'Import bookmarks from html file'")
}

pub fn execute(args: &ArgMatches) {
    let db = DB::open();
    let path = Path::new(args.value_of("FILE").unwrap());
    let bookmarks = get_bookmarks_from_html(path.to_path_buf());

    for bookmark in bookmarks {
        match db.add_bookmark(&bookmark.title, &bookmark.url) {
            Ok(_) => {},
            Err(e) => {
                println!("{} for \"{}\"\n", e, &bookmark.url);
                continue;
            }
        }

        for tag in &bookmark.tags {
            db.add_tag(bookmark.id, tag);
        }

        bookmark.print();
    }
}
