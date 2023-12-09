extern crate logger;
extern crate db;

#[tokio::main]
async fn main() {
    logger::init();
    logger::info("import ui has started..."); 

    print_info();

    db::mongo::init().await;

    logger::info("import ui has done!");
}

fn print_info() {
    logger::info(&format!("logger version is: {}", logger::get_version()));
    logger::info(&format!("logger version is: {}", db::get_db_version()));
}

