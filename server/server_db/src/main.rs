#![deny(warnings)]

use clap::Parser;
use common::{constants::SERVER_DB_APP_NAME, file_helper::CONFIG_EXTENSION};

extern crate common;
extern crate logger;

#[derive(Parser, Debug, Clone)]
#[command(
    name = SERVER_DB_APP_NAME,
    author = "Ali Bigdeli <alibigdeli66@gmail.com>",
    about = "MED client actions are  handled in this part"
)]
struct ServerDbArgs {
    #[arg(
        name = "Well ID",
        short = 'i',
        long = "wellid",
        help = "Well id is a number with 10th lenght"
    )]
    wellid: String,
    #[arg(
        name = "Well Name",
        short = 'n',
        long = "wellname",
        help = "well name is using for loggers, config files, etc."
    )]
    wellname: String,
    #[arg(
        name = "Mongodb Connection String",
        short = 'd',
        long = "connectionstr",
        help = "MongoDB connection string for both Single and Repicasetmode"
    )]
    connectionstring: String,
    #[arg(
        name = "ActiveMQ URL",
        short = 'a',
        long = "activemq",
        help = "Client's Active MQ url"
    )]
    activemqurl: String,
    #[arg(
        name = "MQ Name",
        short = 'm',
        long = "mqname",
        help = "Client's active MQ name"
    )]
    activemqname: String,
}

#[tokio::main]
async fn main() {
    let args = ServerDbArgs::parse();
    common::file_helper::change_working_directory();

    logger::init_with_name(format!("{}{}", args.wellname.trim(), CONFIG_EXTENSION).as_str());

    logger::info("server started");
    print_pargs_info(args.clone());

    logger::error("DANGER ==> server closed !<==");
}

fn print_pargs_info(args: ServerDbArgs) {
    logger::info(format!("Well ID '{}'", args.wellid).as_str());
    logger::info(format!("Well Name '{}'", args.wellname.trim()).as_str());
    logger::info(format!("MongoDb Connection '{}'", args.connectionstring).as_str());
    logger::info(format!("Active MQ URL '{}'", args.activemqurl).as_str());
    logger::info(format!("MQ name is '{}'", args.activemqname).as_str());
}
