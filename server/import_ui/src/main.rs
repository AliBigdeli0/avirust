#![deny(warnings)]

use argparse::{ArgumentParser, StoreTrue};
use import_ui::{start, ParserResult};

extern crate db;
extern crate logger;

mod import_ui;

#[tokio::main]
async fn main() {
    let args = check_args();
    start(args).await;
}

fn check_args() -> ParserResult {
    let mut check = false;
    let mut start: bool = false;
    let mut delete: bool = false;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("use following commands to parser data");

        parser.refer(&mut check).add_option(
            &["-c", "--check"],
            StoreTrue,
            "Check files and create a config file",
        );

        parser.refer(&mut start).add_option(
            &["-s", "--start"],
            StoreTrue,
            "Start write data into database",
        );

        parser.refer(&mut delete).add_option(
            &["-d", "--delete_database"],
            StoreTrue,
            "Clean all excel file content from database",
        );

        parser.parse_args_or_exit();
    }

    ParserResult {
        check_file: check,
        start_program: start,
        delete_database: delete,
    }
}
