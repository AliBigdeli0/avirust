
use import_ui::{start, ParserResult};
use argparse::{ArgumentParser, StoreTrue};

extern crate logger;
extern crate db;

mod import_ui;

#[tokio::main]
async fn main() {
    let args = check_args();
    start(args).await;    
}


fn check_args() -> ParserResult {
    let mut check = false;
    {
        let mut parser = ArgumentParser::new(); 
        parser.set_description("use following commands to parser data"); 

        parser.refer(&mut check)
            .add_option(&["-c", "--check"], StoreTrue, "Check files and create a config file");
        parser.parse_args_or_exit()
    }

    ParserResult { check_file: check }
}

