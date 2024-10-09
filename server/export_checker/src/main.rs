#![deny(warnings)]

extern crate logger;

mod tag_reader;

pub fn get_version() -> String {
    "1.0.0".into()
}

#[tokio::main]
async fn main() {
    init().await;

    tag_reader::start().await;
}

async fn init() {
    logger::init_with_name("export_checker.log");
    logger::info("the logger has been initiated...");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_version() {
        let ver = get_version();
        assert_eq!(ver, "1.0.0");
    }
}
