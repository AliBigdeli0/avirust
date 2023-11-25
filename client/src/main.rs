
fn main() {
    println!("going to startup the program...");
    println!("common version lib : {}", common::get_lib_version());
    println!("working directory is: {}", common::file_helper::current_path().to_str().unwrap());
    println!("configure directory is: {}", common::file_helper::config_path().to_str().unwrap());
}
