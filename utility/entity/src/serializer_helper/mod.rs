use bson::Bson;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

pub use serde_json::{from_str, to_string};

pub fn serialize_json<T>(obj: &T) -> Result<String, serde_json::Error>
where
    T: ?Sized + Serialize,
{
    let string_item = to_string(obj)?;
    Ok(string_item)
}

pub fn deserilize_json<'a, T>(entity_str: &'a str) -> Result<T, serde_json::Error>
where
    T: Deserialize<'a>,
{
    let deser: T = from_str(entity_str)?;
    Ok(deser)
}

pub fn serialize_yaml<T>(entity: &T) -> Result<String, serde_yaml::Error>
where
    T: ?Sized + Serialize,
{
    let string_item = serde_yaml::to_string(entity)?;
    Ok(string_item)
}

pub fn deserilize_yaml<T>(entity_str: &mut str) -> Result<T, serde_yaml::Error>
where
    T: DeserializeOwned,
{
    let deser: T = serde_yaml::from_str(&entity_str)?;
    Ok(deser)
}

pub fn serialize_bson<T>(entity: &T) -> Result<Bson, bson::error::Error>
where
    T: ?Sized + Serialize,
{
    let obj = bson::serialize_to_bson(entity)?;
    Ok(obj)
}

pub fn deserilize_bson<T>(entity_bson: Bson) -> Result<T, bson::error::Error>
where
    T: DeserializeOwned,
{
    bson::deserialize_from_bson(entity_bson)
}

pub fn write_in_file_if_not_exist<T>(path: PathBuf, instance: &T) -> Result<(), serde_yaml::Error>
where
    T: ?Sized + Serialize,
{
    if !common::file_helper::file_exist(path.clone()) {
        let serialized_config = serialize_yaml(&instance).unwrap();

        let mut file = File::create(path.clone()).unwrap();
        let bytes = serialized_config.as_bytes();
        let _ = file.write(bytes);
    }
    Ok(())
}

pub fn read_from_file_lifetime<'a, T>(path: PathBuf) -> Result<T, serde_yaml::Error>
where
    T: DeserializeOwned,
{
    let mut file = File::open(path.clone()).unwrap();

    let mut yaml_string: String = String::from("");
    file.read_to_string(&mut yaml_string).unwrap();

    let instance: T = deserilize_yaml(&mut yaml_string)?;
    Ok(instance)
}

pub fn read_from_file<T>(path: PathBuf) -> T
where
    T: DeserializeOwned,
{
    let mut file = File::open(path.clone()).unwrap();

    let mut yaml_string: String = String::from("");
    file.read_to_string(&mut yaml_string).unwrap();

    let instance: T = deserilize_yaml(&mut yaml_string).unwrap();
    instance
}

pub fn fast_serialize_dic<'a>(dictionary: &HashMap<String, String>) -> std::io::Result<Vec<u8>> {
    let mut writer_buffer: Vec<u8> = Vec::new();

    {
        let mut writer = std::io::BufWriter::new(&mut writer_buffer);

        // write header
        writer.write_all(&(dictionary.len() as u32).to_le_bytes())?;

        for (key, value) in dictionary {
            // write key
            writer.write_all(&(key.len() as i8).to_le_bytes())?;
            writer.write_all(key.as_bytes())?;

            // write value
            writer.write_all(&(value.len() as i8).to_le_bytes())?;
            writer.write_all(value.as_bytes())?;
        }

        writer.flush()?;
    }

    Ok(writer_buffer.clone())
}

pub fn fast_deserialize_dic(bytes: &Vec<u8>) -> std::io::Result<HashMap<String, String>> {
    let mut reader: std::io::BufReader<&[u8]> = BufReader::new(bytes.as_slice());
    let mut ret_map = std::collections::HashMap::new();
    {
        let count = reverse_bytes(&mut reader);

        for _ in 0..count {
            let key = extract_data(&mut reader);
            let value = extract_data(&mut reader);

            ret_map.insert(key, value);
        }
    }

    Ok(ret_map)
}

fn extract_data(reader: &mut std::io::BufReader<&[u8]>) -> String {
    let mut buf_len = [0; 1];
    reader.read_exact(&mut buf_len).unwrap();
    let data_len = buf_len[0] as usize;

    let mut data_bytes = vec![0; data_len as usize];
    reader.read_exact(&mut data_bytes).unwrap();

    String::from_utf8(data_bytes).unwrap()
}

fn reverse_bytes(reader: &mut std::io::BufReader<&[u8]>) -> u32 {
    let mut buf_len = [0; 4];
    reader.read_exact(&mut buf_len).unwrap();
    u32::from_le_bytes(buf_len)
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

    #[derive(Debug, Serialize, Deserialize)]
    struct BsonUserMock {
        name: String,
        password: String,
    }

    fn generate_mock_codes() -> Box<HashMap<String, String>> {
        let mut hash: HashMap<String, String> = HashMap::new();
        for i in 0..512 {
            hash.insert(format!("{:04}",i), format!("{}_bbbbbbbbbbbbbbbbbbbbbbbbbbbbbiiiiiiiiiiiiiiiiiiiiiiiiigggggggggggggggggggggggggssssssssttttttttttttrrrrrrrrrriiiiiiiiiiiinnnnnnnnnnnnnnnnnnngggggggggggggggggggggg", i));
        }
        Box::new(hash)
    }

    #[test]
    fn test_serialize_to_string() {
        let user = UserMock {
            name: "ali".to_string(),
            password: "amir".to_string(),
            age: 10,
            nested: UserNested {
                id: 10,
                string: String::from("..."),
            },
        };
        let serilizad_data = serialize_json(&user).unwrap();
        assert!(serilizad_data.contains("ali"));
        assert!(serilizad_data.contains("amir"));
        assert!(serilizad_data.contains("10"));
    }

    #[test]
    fn test_deserialize_from_string() {
        let user = UserMock {
            name: "hani".to_string(),
            password: "madar".to_string(),
            age: 20,
            nested: UserNested {
                id: 10,
                string: String::from("..."),
            },
        };

        let serialized_data = serialize_json(&user).unwrap();
        let deserliazlied_user: UserMock = deserilize_json(&serialized_data).unwrap();

        assert_eq!(deserliazlied_user.name, "hani");
        assert_eq!(deserliazlied_user.password, "madar");
        assert_eq!(deserliazlied_user.age, 20);
    }

    #[test]
    fn yaml_serialize_string() {
        let user = UserMock {
            name: "ali".to_string(),
            password: "amir".to_string(),
            age: 10,
            nested: UserNested {
                id: 122,
                string: String::from("..."),
            },
        };
        let mut serialized_data = serialize_yaml(&user).unwrap();
        let deserialized_data: UserMock = deserilize_yaml(&mut serialized_data).unwrap();

        assert_eq!(deserialized_data.name, user.name);
        assert_eq!(deserialized_data.nested.id, user.nested.id);
    }

    #[test]
    fn bson_seralization() {
        let user = BsonUserMock {
            name: "ali".to_string(),
            password: "amir".to_string(),
        };
        let serialized_user = serialize_bson(&user).unwrap();
        println!("the data is: {}", serialized_user);
        let user_back = deserilize_bson::<BsonUserMock>(serialized_user).unwrap();

        assert_eq!(user_back.name, "ali");
        assert_eq!(user_back.password, "amir");
    }

    #[test]
    fn test_fast_serialize() {
        let hash = *generate_mock_codes();
        let buffer = fast_serialize_dic(&hash.clone()).unwrap();

        assert_ne!(buffer.len(), 0);

        let deserialized_hash = fast_deserialize_dic(&buffer).unwrap();

        assert_eq!(deserialized_hash.len(), hash.len());
        for (key, value) in deserialized_hash {
            let original_value = hash.get(&key).unwrap();
            assert_eq!(&value, original_value);
        }
    }
}
