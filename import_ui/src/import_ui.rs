use entity::{dto::excel_dto::ExcelInfo, model::db::Well};
use std::io;
pub struct ParserResult {
    pub check_file: bool, 
}

pub async fn start(parser_result: ParserResult) {    

    init_screen().unwrap();
    logger::init();
    logger::info("import ui has started..."); 
    common::file_helper::import_path();
    
    print_info();
    
    let (_, database) = db::mongo::init().await;


    if parser_result.check_file {
        logger::info("check file names has started"); 

        let wells = db::mongo::read_all_wells(database).await.unwrap();
        let well = print_select_well(&wells).unwrap();
        let import_path = common::file_helper::import_path();
        let file_list = common::file_helper::all_files_in_directory(&import_path, Some("xlsx")).unwrap();
        let mut excel_obj = Box::new(ExcelInfo::new());
        
        logger::info(&format!("number of files in the import directory: [{}]", file_list.len()));
        
        excel_obj.set_well_id(&well.Name);

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
        
    }
    logger::info("import data has done!");
}

fn print_select_well(wells: &Vec<Well>) -> Option<&Well> {
    let mut input_string = String::new();

    for (index, well) in wells.iter().enumerate() {
        println!("{}). {} ({})", index + 1, well.Name, well.ID);
    }
    println!("select a number from following well lists>>> ");

    io::stdin().read_line(&mut input_string).expect("failed to read line");

    let selected_index = input_string.trim().parse::<usize>().expect("can not convert entered number to integer"); 
    wells.get(selected_index - 1)
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
