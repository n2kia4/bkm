use clap::{App, ArgMatches, SubCommand};
use webbrowser;

use database::DB;

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("open")
        .about("Open bookmark")
        .arg_from_usage("<ID>... 'Open bookmark matching the specified ids'")
}

pub fn execute(args: &ArgMatches) {
    let db = DB::open();

    let ids = values_t!(args, "ID", i64).unwrap();

    for id in ids {
        webbrowser::open(&db.get_url_by_id(id)).unwrap();
    }
}
