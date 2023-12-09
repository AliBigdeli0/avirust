

pub struct ParserResult {
    pub check_file: bool, 
}

pub async fn start(parser_result: ParserResult) {    
    logger::init();
    logger::info("import ui has started..."); 
    common::file_helper::import_path();

    if parser_result.check_file {
        logger::info("check file names has started"); 
        let import_path = common::file_helper::import_path();
        let file_list = common::file_helper::all_files_in_directory(&import_path, Some("dat")).unwrap();
        
        for file in file_list {
            println!("the file name is: {:?}" , file);
        }

        // break control
        std::process::exit(0);
    }
    else {
        print_info();
        db::mongo::init().await;
    }
    logger::info("import ui has done!");
}

fn print_info() {
    logger::info(&format!("logger version is: {}", logger::get_version()));
    logger::info(&format!("logger version is: {}", db::get_db_version()))
}
