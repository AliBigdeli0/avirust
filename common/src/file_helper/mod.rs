use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

pub fn current_path() -> PathBuf {
    current_dir().unwrap()
}

pub fn config_path() -> PathBuf {
    let buff = current_path().join("config");
    create_dir_if_not_exist(buff.as_os_str().to_str().unwrap()).unwrap();
    buff
}

pub fn create_dir_if_not_exist(new_path : &str) -> Result<(), std::io::Error> {
    if !fs::metadata(new_path).is_ok() {
        fs::create_dir_all(new_path)?;
    }
    Ok(())
}

#[cfg(test)]
mod integration_test {
    use super::*;
    fn setup() {
        let cu_path = PathBuf::from("./target/");
        let _ = std::env::set_current_dir(cu_path.as_path());
    }

    #[test]
    fn  test_create_config_dir() {
        setup();
        match create_dir_if_not_exist("config") {
            Ok(())=> {},
            Err(err) => { panic!("{}",err); }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_current_path() {
        let buff= current_path();
        assert_ne!(buff.as_os_str(), "");
    }

    #[test]
    fn test_config_path() {
        let c_buff = config_path();
        let config_path_str = String::from(c_buff.to_str().unwrap());
        println!("{}",config_path_str);
        assert!(config_path_str.ends_with("config"))
    }
}