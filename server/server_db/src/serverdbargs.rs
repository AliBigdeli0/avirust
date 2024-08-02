use clap::Parser;
use common::constants::SERVER_DB_APP_NAME;

#[derive(Parser, Debug, Clone)]
#[command(
    name = SERVER_DB_APP_NAME,
    author = "Ali Bigdeli <alibigdeli66@gmail.com>",
    about = "MED client actions are  handled in this part"
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
        name = "Well Name",
        short = 'n',
        long = "wellname",
        help = "well name is using for loggers, config files, etc."
    )]
    pub wellname: String,
    #[arg(
        name = "Mongodb Connection String",
        short = 'd',
        long = "connectionstr",
        help = "MongoDB connection string for both Single and Repicasetmode"
    )]
    pub connectionstring: String,
    #[arg(
        name = "ActiveMQ URL",
        short = 'a',
        long = "activemq",
        help = "Client's Active MQ url"
    )]
    pub activemqurl: String,
    #[arg(
        name = "MQ Name",
        short = 'm',
        long = "mqname",
        help = "Client's active MQ name"
    )]
    pub activemqname: String,
}
