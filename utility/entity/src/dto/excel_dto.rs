use std::collections::HashMap;
use std::path::PathBuf;
use std::vec;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use super::super::serializer_helper::write_in_file_if_not_exist;

pub const IGNORE_CODE: &str = "-";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExcelColumnInfo {
    pub column: String,
    pub code: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub hash: u64,
    #[serde(skip_serializing, skip_deserializing)]
    pub value: String,
}

impl ExcelColumnInfo {
    pub fn new(column_value: String, code_value: String) -> Self {
        let hash_value = common::hash_helper::hash(&column_value);

        Self {
            column: column_value,
            code: code_value,
            hash: hash_value,
            value: String::from(""),
        }
    }
}

// ----------------------------------
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExcelInfo {
    well_id: String,
    well_name: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub date: DateTime<Utc>,
    pub list_items: Vec<ExcelColumnInfo>,
}

impl ExcelInfo {
    pub fn new() -> Self {
        Self {
            well_id: String::from(""),
            well_name: String::from(""),
            date: common::time_helper::get_now_utc(),
            list_items: vec![],
        }
    }

    pub fn set_well_id(&mut self, new_well_id: &str) {
        self.well_id = new_well_id.to_string();
    }

    pub fn get_well_id(&self) -> &String {
        &self.well_id
    }

    pub fn set_date(&mut self, date: DateTime<Utc>) {
        self.date = date;
    }

    pub fn set_value(&mut self, index: usize, value: String) {
        if !value.trim().is_empty() {
            self.list_items[index].value = value;
        }
    }

    pub fn append(&mut self, column: String, code: String) {
        self.append_value(column, code, None)
    }

    pub fn append_value(&mut self, column: String, code: String, value: Option<String>) {
        let mut excel_info = ExcelColumnInfo::new(column, code);

        if let Some(uv) = value {
            excel_info.value = uv;
        }

        if !self.list_items.iter().any(|x| x.hash == excel_info.hash) {
            self.list_items.push(excel_info);
        }
    }

    pub fn size(&self) -> usize {
        self.list_items.len()
    }

    pub fn write_into_file(&mut self) {
        let excel_file_name = common::file_helper::generate_yaml_extention(
            format!("{}_{}", self.well_name, self.well_id)
        );
        let excel_file_path_buff = PathBuf::from(&excel_file_name);
        let excel_full_path = common::file_helper::import_path().join(excel_file_path_buff);
        write_in_file_if_not_exist(excel_full_path, &self).unwrap();
    }

    pub fn set_well_name(&mut self, well_name: &str) {
        self.well_name = well_name.to_string();
    }

    pub fn get_well_name(&self) -> &str {
        self.well_name.as_ref()
    }

    pub fn to_hash(&self) -> HashMap<String, String> {
        self.list_items
            .clone()
            .into_iter()
            .filter(|x| x.code != IGNORE_CODE)
            .fold(HashMap::new(), |mut hash, item| {
                hash.insert(item.code, item.value);
                hash
            })
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

    #[test]
    fn test_to_hash() {
        let mut mock_excel_info = ExcelInfo::new();

        mock_excel_info.append_value(
            "ali".to_string(),
            "mehran".to_string(),
            Some("value".to_string())
        );

        mock_excel_info.append("amir".to_string(), "hani".to_string());
        mock_excel_info.append("jarandee".to_string(), "asefeh".to_string());
        mock_excel_info.append("maman".to_string(), "mina".to_string());

        let hash = mock_excel_info.to_hash();
        assert_eq!(hash.len(), mock_excel_info.list_items.len());
        assert_eq!(hash.get("mehran").unwrap(), &mock_excel_info.list_items.get(0).unwrap().value);

        mock_excel_info.append_value(
            "ignored_value".to_string(),
            IGNORE_CODE.to_string(),
            Some("new_value".to_string())
        );

        let ignored_hash = mock_excel_info.to_hash();

        println!("{:?}", ignored_hash);
        assert_eq!(ignored_hash.len(), mock_excel_info.list_items.len() - 1);
    }
}
