use std::env::current_dir;
use std::fs::{self, DirEntry, ReadDir};
use std::path::PathBuf;

pub const EXCEL_FILE_EXTENSION: &str = "xls";
pub const YAML_FILE_EXTENSION: &str = "yaml";
pub const CONFIG_EXTENSION: &str = ".log";

pub fn current_path() -> PathBuf {
    current_dir().unwrap()
}

pub fn config_path() -> PathBuf {
    let buff = current_path().join("config");
    create_dir_if_not_exist(buff.as_os_str().to_str().unwrap()).unwrap();
    buff
}

pub fn import_path() -> PathBuf {
    let import_buff = current_path().join("import");
    create_dir_if_not_exist(import_buff.as_os_str().to_str().unwrap()).unwrap();
    import_buff
}

pub fn log4rs_yaml_path(file_name: Option<&str>) -> PathBuf {
    match file_name {
        Some(fname) => config_path().join(format!(
            "log4rs_{}.{}",
            fname.replace(CONFIG_EXTENSION, ""),
            YAML_FILE_EXTENSION
        )),
        None => config_path().join("log4rs.yaml"),
    }
}

pub fn server_yaml_path() -> PathBuf {
    config_path().join("server.yaml")
}

pub fn db_yaml_path() -> PathBuf {
    config_path().join("db.yaml")
}

pub fn ui_config_yaml_path() -> PathBuf {
    config_path().join("ui_config.yaml")
}

pub fn generate_yaml_extention(file_name: String) -> String {
    format!("{}.{}", file_name.as_str(), YAML_FILE_EXTENSION)
}

pub fn log_file_path() -> PathBuf {
    let buff = current_path().join("logs");
    create_dir_if_not_exist(buff.as_os_str().to_str().unwrap()).unwrap();
    buff.join("luncher.log")
}

pub fn create_dir_if_not_exist(new_path: &str) -> Result<(), std::io::Error> {
    if !fs::metadata(new_path).is_ok() {
        fs::create_dir_all(new_path)?;
    }
    Ok(())
}

pub fn file_exist(file_path: PathBuf) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        metadata.is_file()
    } else {
        false
    }
}

#[derive(Debug)]
pub enum AllFilesInDirectoryError {
    IOError(std::io::Error),
    STDError(String),
}

pub fn all_files_in_directory<'a>(
    directory_path: &PathBuf,
    extention: Option<&str>,
) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut list_files: Vec<PathBuf> = vec![];
    let meta_data = fs::metadata(directory_path);
    if !meta_data.is_ok() {
        return Err(meta_data.err().unwrap());
    }

    let fs_data = meta_data.ok().unwrap();
    if !fs_data.is_dir() {
        return Ok(list_files);
    }

    let entries: ReadDir = fs::read_dir(directory_path).unwrap();

    for entry in entries {
        let entry: DirEntry = entry.unwrap();
        let file_path: PathBuf = entry.path();

        if file_path.is_file() {
            match extention {
                Some(value) => {
                    let file_ext = file_path.extension().unwrap().to_str().unwrap();
                    if file_ext.to_lowercase() == value.to_lowercase() {
                        list_files.push(file_path.clone())
                    }
                }
                None => list_files.push(file_path.clone()),
            }
        }
    }

    Ok(list_files)
}

pub fn change_working_directory() {
    let current_path = current_path();
    std::env::set_current_dir(current_path.clone()).unwrap();
    println!("working directory is: {:?}", current_path);
}

#[cfg(test)]
mod integration_test {
    use super::*;

    fn setup() {
        let cu_path = PathBuf::from("./target/");
        let _ = std::env::set_current_dir(cu_path.as_path());
    }

    #[test]
    fn test_create_config_dir() {
        setup();
        match create_dir_if_not_exist("config") {
            Ok(()) => {}
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_current_path() {
        let buff = current_path();
        assert_ne!(buff.as_os_str(), "");
    }

    #[test]
    fn test_config_path() {
        let c_buff = config_path();
        let config_path_str = String::from(c_buff.to_str().unwrap());
        println!("{}", config_path_str);
        assert!(config_path_str.ends_with("config"))
    }

    #[test]
    fn generate_yaml() {
        let file_name = generate_yaml_extention(String::from("ali"));
        assert!(file_name.ends_with(".yaml"))
    }
}
