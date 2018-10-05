use clap::{App, Arg, ArgMatches, SubCommand};
use std::{io, process};

use database::DB;

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("delete")
        .about("Delete bookmark")
        .arg(Arg::from_usage("<ID>... 'Delete bookmarks matching the specified ids{n}\
                             (If id is not specified, delete all bookmarks)'").required(false))
        .arg_from_usage("-T 'Delete all tags'")
        .arg_from_usage("-t --tag_id=[id]... 'Delete tags matching the specified ids'")
}

pub fn execute(args: &ArgMatches) {
    let db = DB::open();

    if args.is_present("T") {
        yes_or_no("tags");

        db.clear("tags");
        db.clear("bookmark_tag");

        process::exit(0);
    }

    if args.is_present("tag_id") {
        let ids = values_t!(args, "tag_id", i64).unwrap();
        for id in ids {
            let result: i64 = db.check_existence("tags", id);
            if result == 1 {
                db.delete("tags", id);
                db.delete_bookmark_tag("tag_id", id);
                println!("Index {} deleted", id);
            } else {
                println!("Error: No match index");
            }
        }

        process::exit(0);
    }

    if !args.is_present("ID") {
        yes_or_no("bookmarks");

        db.clear("bookmarks");
        db.clear("tags");
        db.clear("bookmark_tag");

        process::exit(0);
    }

    let ids = values_t!(args, "ID", i64).unwrap();
    for id in ids {
        let result: i64 = db.check_existence("bookmarks", id);
        if result == 1 {
            db.delete("bookmarks", id);
            db.delete_bookmark_tag("bookmark_id", id);
            println!("Index {} deleted", id);
        } else {
            println!("Error: No match index");
        }
    }
}

fn yes_or_no(table_name: &str) {
    loop {
        println!("Delete all {}? (y/n)", table_name);

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        let answer = answer.trim();

        match answer {
            "Y" | "y" | "" => println!("Done!"),
            "N" | "n" => process::exit(1),
            _ => continue,
        }

        break;
    }
}
