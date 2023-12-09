use entity::dto::excel_dto::ExcelInfo;

pub struct ParserResult {
    pub check_file: bool, 
}

pub async fn start(parser_result: ParserResult) {    

    init_screen().unwrap();
    logger::init();
    logger::info("import ui has started..."); 
    common::file_helper::import_path();
    
    print_info();
    
    if parser_result.check_file {
        logger::info("check file names has started"); 
        let import_path = common::file_helper::import_path();
        let file_list = common::file_helper::all_files_in_directory(&import_path, Some("xlsx")).unwrap();
        
        logger::info(&format!("number of files in the import directory: [{}]", file_list.len()));
        
        println!("");

        let mut excel_obj = Box::new(ExcelInfo::new());
        excel_obj.set_well_id("test_well");

        for (index,file_path) in file_list.iter().enumerate() {
            if let Some(path_str) = file_path.to_str() {
                print!("\r file [{}]: {:?}", index, path_str); 
                let _ = excel::excel_helper::read_excel_info(Box::new(path_str), &mut excel_obj).unwrap();
            }
        }

        excel_obj.write_into_file();
        
        std::process::exit(0);
    }
    else {
        db::mongo::init().await;
    }
    logger::info("import data has done!");
}

fn init_screen() -> std::io::Result<()>{
    Ok(())
}

fn print_info() {
    logger::info("======================");
    logger::info(&format!("logger version is: {}", logger::get_version()));
    logger::info(&format!("db version is: {}", db::get_db_version()));
    logger::info(&format!("excel version is: {}", excel::get_excel_version()));
    logger::info("======================");
}
