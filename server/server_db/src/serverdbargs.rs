use clap::Parser;
use common::constants::SERVER_DB_APP_NAME;

#[derive(Parser, Debug, Clone)]
#[command(
    name = SERVER_DB_APP_NAME,
    author = "Ali Bigdeli <alibigdeli66@gmail.com>",
    about = "Thie program is reading from client's MQ in STOMP standard."
)]
pub struct ServerDbArgs {
    #[arg(
        name = "Well ID",
        short = 'i',
        long = "wellid",
        help = "Well id is a number with 10th lenght"
    )]
    pub wellid: String,
    #[arg(
        name = "Mongodb Connection String",
        short = 'd',
        long = "connectionstr",
        help = "MongoDB connection string for both Single and Repicasetmode"
    )]
    pub connectionstring: String,
}
