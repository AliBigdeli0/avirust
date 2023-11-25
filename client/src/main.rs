
fn main() {
    print_info();
}

fn print_info() {
    println!("going to startup the program...");

    // libs
    println!("common version lib : {}", common::get_lib_version());
    println!("entity version is: {}", entity::get_version());

    // directory ifo
    println!("working directory is: {}", common::file_helper::current_path().to_str().unwrap());
    println!("configure directory is: {}", common::file_helper::config_path().to_str().unwrap());
}
