use std::{path::PathBuf, str::FromStr};

use common::file_helper;

pub async fn start() {
    logger::info("goint to start ...");
    let tags_dir = PathBuf::from_str("tags").unwrap();
    let tag_files_result = file_helper::all_files_in_directory(&tags_dir, Some("TAG"));

    match tag_files_result {
        Ok(tag_files) => navigate_files(tag_files),
        Err(err) => logger::error(&format!("error in read from tag files : {}", err)),
    }

    logger::info("finished!!!");
}

fn navigate_files(tag_files: Vec<PathBuf>) {
    for file in tag_files {
        let file_name = file.to_str().unwrap();

        logger::info(&format!("going to parse : {}", file_name));
    }
}
