use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
pub use serde_json::{from_str, to_string};

pub fn serialize_json<T>(obj: &T) -> Result<String, serde_json::Error> where
    T: ?Sized + Serialize,
{
    let string_item = to_string(obj)?;
    Ok(string_item)
}

pub fn deserilize_json<'a, T>(entity_str: &'a str) -> Result<T, serde_json::Error> where T: Deserialize<'a>,
{
    let deser: T = from_str(entity_str)?;
    Ok(deser)
}

pub fn serialize_yaml<T>(entity: &T) -> Result<String, serde_yaml::Error> where
    T: ?Sized + Serialize,
{
    let string_item = serde_yaml::to_string(entity)?;
    Ok(string_item)
}

pub fn deserilize_yaml<T>(entity_str: &mut str) -> Result<T, serde_yaml::Error> where T: DeserializeOwned,
{
    let deser: T = serde_yaml::from_str(&entity_str)?;
    Ok(deser)
}

pub fn write_in_file_if_not_exist<T>(path: PathBuf, instance: &T) -> Result<(), serde_yaml::Error>
    where T: ?Sized + Serialize
{
    if !common::file_helper::file_exist(path.clone()) {
        let serialized_config = serialize_yaml(&instance).unwrap();

        let mut file = File::create(path.clone()).unwrap();
        let bytes = serialized_config.as_bytes();
        let _ = file.write(bytes);
    }
    Ok(())
}


pub fn read_from_file<T>(path: PathBuf) -> T 
where T: DeserializeOwned,
{
    let mut file = File::open(path.clone()).unwrap();

    let mut  yaml_string: String = String::from("");
    file.read_to_string(&mut yaml_string).unwrap();

    let instance: T = deserilize_yaml(&mut yaml_string).unwrap();
    instance
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct UserNested {
        id: i32,
        string: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct UserMock {
        name: String,
        password: String,
        age: i32,
        nested: UserNested,
    }

    #[test]
    fn test_serialize_to_string() {
        let user = UserMock { name: "ali".to_string(), password: "amir".to_string(), age: 10, nested: UserNested { id: 10, string: String::from("...") } };
        let serilizad_data = serialize_json(&user).unwrap();
        assert!(serilizad_data.contains("ali"));
        assert!(serilizad_data.contains("amir"));
        assert!(serilizad_data.contains("10"));
    }

    #[test]
    fn test_deserialize_from_string() {
        let user = UserMock { name: "hani".to_string(), password: "madar".to_string(), age: 20, nested: UserNested { id: 10, string: String::from("...") } };

        let serialized_data = serialize_json(&user).unwrap();
        let deserliazlied_user: UserMock = deserilize_json(&serialized_data).unwrap();

        assert_eq!(deserliazlied_user.name, "hani");
        assert_eq!(deserliazlied_user.password, "madar");
        assert_eq!(deserliazlied_user.age, 20);
    }

    #[test]
    fn yaml_serialize_string() {
        let user = UserMock { name: "ali".to_string(), password: "amir".to_string(), age: 10, nested: UserNested { id: 122, string: String::from("...") } };
        let mut serialized_data = serialize_yaml(&user).unwrap();

        println!("{}", &mut serialized_data);

        let deserialized_data: UserMock = deserilize_yaml(&mut serialized_data).unwrap();

        assert_eq!(deserialized_data.name, user.name);
        assert_eq!(deserialized_data.nested.id, user.nested.id);
    }
}