use std::path::PathBuf;

use common::{file_helper::*, print_helper::*};
use entity::{dto::excel_dto::ExcelInfo, model::db::Well, serializer_helper::read_from_file};
use excel::excel_helper::{check_configurations, read_excel_info};
use mongodb::Database;

pub struct ParserResult {
    pub check_file: bool,
    pub start_program: bool,
}

pub async fn start(parser_result: ParserResult) {
    logger::init();
    logger::info("import ui has started...");

    //init import path
    common::file_helper::import_path();

    print_info();

    // init database
    let (_, database) = db::mongo::init().await;

    //test read from database
    let collection_name = "c_2345415781_23_10_23";
    let _ = db::mongo::read_from_time_data(database.clone(), collection_name)
        .await
        .unwrap();
    // let hash_list = time_list.get(0).unwrap().data_to_hash();
    // let mut sorted_entries: Vec<_> = hash_list.into_iter().collect();
    // sorted_entries.sort_by(|(a,_), (b,_)| a.cmp(&b));

    // for item in sorted_entries {
    //     println!("{}:{}", item.0, item.1)
    // }

    // if check file switched
    if parser_result.check_file {
        logger::info("check file names has started");

        let well = select_well_from_wells(database).await;

        let file_list = read_all_excel_files();

        let mut excel_obj = excel_object_builder(well);

        parse_files_into_excel_obj(file_list, &mut excel_obj);

        excel_obj.write_into_file();
    } else if parser_result.start_program {
        let selected_file = select_from_one_yaml_file();

        println!("the selected file is: {:?}", selected_file);

        let excel_config: &ExcelInfo = &read_from_file(*selected_file.to_owned());
        print_excel_obj_info(excel_config);

        let ch = print_and_confirm_start_program();

        if ch == YES {
            successln("confirmed, going to parse configurations");

            let valid_configurations = match check_configurations(excel_config) {
                Ok(str) => str,
                Err(err) => {
                    errorln(err.as_str());
                    panic!();
                }
            };

            extract_and_confirm_again(valid_configurations);

            let xls_files = read_all_excel_files();

            for xlc_file in xls_files {
                {
                    println!("{:?}", xlc_file);
                }
            }
        } else {
            errorln("your answer is considered as NO! please write a capital letter 'Y' if you mean 'yes'");
        }
    }

    successln("DONE!");
    logger::info("import data has done!");
}

fn extract_and_confirm_again(valid_configurations: String) {
    warningln("the valid code items are :");
    warningln(&valid_configurations);
    let answser = confirm_message("Are you sure about that?");
    if answser == NO {
        errorln("you are not confirmed the configurations, please check it again");
        panic!()
    }
}

fn print_excel_obj_info(excel_config: &ExcelInfo) {
    println!();
    println!();
    warningln(&format!(
        "well info=> name: {}, ID:{}",
        excel_config.get_well_name(),
        excel_config.get_well_id()
    ));
}

fn print_and_confirm_start_program() -> char {
    successln("Please check the above information");
    warningln("WARNING: all stored data CAN NOT be recovered");
    let ch = confirm_message("are you sure?");
    ch
}

fn select_from_one_yaml_file<'a>() -> Box<PathBuf> {
    let import_path = import_path();
    let yaml_files: Vec<PathBuf> =
        all_files_in_directory(&import_path, Some(YAML_FILE_EXTENSION)).unwrap();
    let selected_file: &PathBuf = select_from_list(&yaml_files, |index, file| {
        if let Some(file_name) = file.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                println!("{}. {:?}", index + 1, file_name_str);
            }
        }
    })
    .unwrap();
    Box::new(selected_file.clone())
}

fn parse_files_into_excel_obj(file_list: Vec<std::path::PathBuf>, excel_obj: &mut Box<ExcelInfo>) {
    for (index, file_path) in file_list.iter().enumerate() {
        if let Some(path_str) = file_path.to_str() {
            print!("\r file [{}]: {:?}", index, path_str);
            let _ = read_excel_info(Box::new(path_str), excel_obj).unwrap();
        }
    }
}

fn read_all_excel_files() -> Vec<std::path::PathBuf> {
    let import_path = import_path();
    let file_list = all_files_in_directory(&import_path, Some(EXCEL_FILE_EXTENSION)).unwrap();
    logger::info(&format!(
        "number of excel files in the import directory: [{}]",
        file_list.len()
    ));
    file_list
}

fn excel_object_builder(well: &Well) -> Box<ExcelInfo> {
    let mut excel_obj = Box::new(ExcelInfo::new());
    excel_obj.set_well_id(&well.id);
    excel_obj.set_well_name(&well.name);
    excel_obj
}

async fn select_well_from_wells<'a>(database: Database) -> &'a Well {
    let wells: &'a Vec<Well> = db::mongo::read_all_wells(database).await.unwrap();
    let well: &'a Well = select_from_list(&wells, |index, well| {
        println!("{}. {} ({})", index + 1, well.name, well.id);
    })
    .unwrap();
    well
}

fn print_info() {
    logger::info("======================");
    logger::info(&format!("logger version is: {}", logger::get_version()));
    logger::info(&format!("db version is: {}", db::get_db_version()));
    logger::info(&format!("excel version is: {}", excel::get_excel_version()));
    logger::info("======================");
}
