fn main() {
    print_info();
}

fn print_info() {
    println!("logger version is: {}", logger::get_version());
    println!("web version is: {}", web::get_version());
    println!("db version is: {}", db::get_db_version());
    println!("excel version is: {}", excel::get_excel_version());
}