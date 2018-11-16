use clap::{App, ArgMatches, SubCommand};

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("import")
        .about("Import bookmark")
}

pub fn execute(args: &ArgMatches) {
}
