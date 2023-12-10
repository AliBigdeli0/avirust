use entity::dto::excel_dto::ExcelInfo;
use std::io;
use common::file_helper::*;

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
        let well = select_from_list(&wells, |index,well| {
            println!("{}. {} ({})", index + 1, well.Name, well.ID);
        }).unwrap();
        let import_path = import_path();
        let file_list = all_files_in_directory(&import_path, Some(EXCEL_FILE_EXTENSION)).unwrap();
        let mut excel_obj = Box::new(ExcelInfo::new());
        
        logger::info(&format!("number of files in the import directory: [{}]", file_list.len()));
        
        excel_obj.set_well_id(&well.ID);
        excel_obj.set_well_name(&well.Name);

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

fn select_from_list<T,F>(list: &Vec<T>,  print_operation: F) -> Option<&T>
    where F: Fn(usize,&T) -> (),
{
    let mut input_string = String::new();

    for (index, item) in list.iter().enumerate() {
        print_operation(index, item);
    }

    print!("select a number from following name: >>>\n"); 

    io::stdin()
        .read_line(&mut input_string)
        .expect("failed to read line");

    let selected_index = input_string
        .trim()
        .parse::<usize>()
        .expect("can not convert entered number to integer"); 

    list.get(selected_index - 1)
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
