fn main() {
    print_info();

    logger::init();
    logger::info("###$#$#$#$#$#$#$#$$$#$#$");
    logger::error("here is");
}

fn print_info() {
    println!("going to startup the program...");

    // libs
    println!("common version is: {}", common::get_lib_version());
    println!("entity version is: {}", entity::get_version());
    println!("logger version is: {}", logger::get_version());

    // directory ifo
    println!("working directory is: {}", common::file_helper::current_path().to_str().unwrap());
    println!("configure directory is: {}", common::file_helper::config_path().to_str().unwrap());
}


#[cfg(test)]
mod test {
    use entity::serializer_helper::serialize;
    use entity::dto::Point;

    #[test]
    fn serialize_out_of_scope() {
        let obj = serialize(&Point { x: 10, y: 20 }).unwrap();
        assert!(obj.contains("10"));
        assert!(obj.contains("20"));
    }
}