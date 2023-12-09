
use std::path::PathBuf;
use std::vec;

use serde::Deserialize;
use serde::Serialize;

use super::super::serializer_helper::write_in_file_if_not_exist;


#[derive(Debug,Deserialize, Serialize)]
pub struct ExcelColumnInfo {
    pub column: String, 
    pub code : String,
    #[serde(skip_serializing)]
    pub hash: u64
}

impl ExcelColumnInfo {
    pub fn new(column_value:String, code_value: String) -> Self {
        let hash_value = common::hash_helper::hash(&column_value);

        Self {
            column : column_value, 
            code: code_value,
            hash: hash_value,
        }
    }
}

// ----------------------------------
#[derive(Debug,Deserialize, Serialize)]
pub struct  ExcelInfo {
    well_id: String,
    list_items: Vec<ExcelColumnInfo>,
}

impl ExcelInfo {
    pub fn new () -> Self {
        Self { 
            well_id: String::from(""), 
            list_items: vec![] 
        }
    }

    pub fn set_well_id(&mut self, new_well_id: &str) {
        self.well_id = new_well_id.to_string();
    }

    pub fn get_well_id(&self) -> &String {
        &self.well_id
    }

    pub fn append(&mut self, column: String, code : String ) {
        let excel_info = ExcelColumnInfo::new(column , code);

        if !self.list_items.iter().any(|x| x.hash == excel_info.hash) {
            self.list_items.push(excel_info);     
        }
    }

    pub fn size(&self) ->usize {
        self.list_items.len()
    }

    pub fn write_into_file(&mut self) {
        let excel_file_name = common::file_helper::generate_yaml_extention(self.get_well_id().to_string());
        let excel_file_path_buff = PathBuf::from(&excel_file_name);
        let excel_full_path = common::file_helper::config_path().join(excel_file_path_buff);
        write_in_file_if_not_exist(excel_full_path, &self).unwrap();
    }
}


#[cfg(test)]
mod test_excel_dto {
    use super::*;

    #[test]
    fn test_push_item() {
        let mut excel_info = ExcelInfo::new();
        excel_info.append(String::from("ali"), String::from("amir"));
        assert_eq!(excel_info.size(), 1);
        excel_info.append(String::from("ali"), String::from("another"));
        assert_eq!(excel_info.size(), 1);
        excel_info.append(String::from("hani"), String::from("amir"));
        assert_eq!(excel_info.size(), 2);
    }
}