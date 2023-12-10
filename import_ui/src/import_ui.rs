use entity::{dto::excel_dto::ExcelInfo, serializer_helper::read_from_file};
use std::io::{self, Write};
use common::{file_helper::*, print_helper::*};
use excel::excel_helper::{check_configurations, read_excel_info};

const YES:char = 'Y'; 
const NO:char = 'N';

pub struct ParserResult {
    pub check_file: bool, 
    pub start_program: bool,
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
            println!("{}. {} ({})", index + 1, well.name, well.id);
        }).unwrap();
        let import_path = import_path();
        let file_list = all_files_in_directory(&import_path, Some(EXCEL_FILE_EXTENSION)).unwrap();
        let mut excel_obj = Box::new(ExcelInfo::new());
        
        logger::info(&format!("number of files in the import directory: [{}]", file_list.len()));
        
        excel_obj.set_well_id(&well.id);
        excel_obj.set_well_name(&well.name);

        for (index,file_path) in file_list.iter().enumerate() {
            if let Some(path_str) = file_path.to_str() {
                print!("\r file [{}]: {:?}", index, path_str); 
                let _ = read_excel_info(Box::new(path_str), &mut excel_obj).unwrap();
            }
        }

        excel_obj.write_into_file();
        
    }
    else if parser_result.start_program  {
        let import_path = import_path();
        let yaml_files = all_files_in_directory(&import_path, Some(YAML_FILE_EXTENSION)).unwrap();
        let selected_file = select_from_list(&yaml_files, |index,file| {
            if let Some(file_name) = file.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    println!("{}. {:?}", index + 1, file_name_str);
                }
            }
        }).unwrap();

        println!("the selected file is: {:?}", selected_file);
        
        let excel_config:ExcelInfo  = read_from_file(selected_file.to_owned());
        println!();
        println!();
        warningln(&format!("well info=> name: {}, ID:{}",excel_config.get_well_name(), excel_config.get_well_id()));

        successln("Please check the above information");
        warningln("WARNING: all stored data CAN NOT be recovered");
        let ch = confirm_message("are you sure?");
        

        if ch == YES {
            successln("confirmed, going to parse configurations");
            let valid_configurations = match check_configurations(excel_config) {
                Ok(str) => str, 
                Err(err) => {
                    errorln(err.as_str());
                    panic!();
                }
            };
            
            warningln("the valid code items are :");
            warningln(&valid_configurations);
            let answser = confirm_message("Are you sure about that?");
            if answser == NO {
                errorln("you are not confirmed the configurations, please check it again");
                panic!()
            }

            let xls_files = all_files_in_directory(&import_path, Some(EXCEL_FILE_EXTENSION)).unwrap();

            for xlc_file in xls_files {
                {
                    println!("{:?}", xlc_file);
                }
            }
        }
        else {
            errorln("your answer is considered as NO! please write a capital letter 'Y' if you mean 'yes'");
        }
    }

    successln("DONE!");
    logger::info("import data has done!");
}

fn confirm_message(msg: &str) -> char {
    print!("{}",msg); 
    warning("(Y/n) "); 
    io::stdout().flush().unwrap();

    let mut input_string = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("failed to read line");

    match input_string.trim() {
        "Y" => YES,
        _ =>  NO
    }
}

fn select_from_list<T,F>(list: &Vec<T>,  print_operation: F) -> Option<&T>
    where F: Fn(usize,&T) -> (),
{
    let mut input_string = String::new();

    for (index, item) in list.iter().enumerate() {
        print_operation(index, item);
    }

    warning("select a number from following name: ? "); 
    io::stdout().flush().unwrap();

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
