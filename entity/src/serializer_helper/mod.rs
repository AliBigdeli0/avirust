use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use bson::Bson;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

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

pub fn deserilize_yaml<T>(entity_str: &mut str) -> Result<T, serde_yaml::Error>
    where T: DeserializeOwned,
{
    let deser: T = serde_yaml::from_str(&entity_str)?;
    Ok(deser)
}

pub fn serialize_bson<T>(entity: &T) -> Result<Bson, bson::ser::Error>
    where T:  ?Sized + Serialize
{
   let obj = bson::to_bson(entity)?;
   Ok(obj)
}

pub fn deserilize_bson<T>(entity_bson: Bson) -> Result<T, bson::de::Error>
    where T: DeserializeOwned,
{
    let entity: T = match bson::from_bson(entity_bson) {
        Ok(doc)=> doc, 
        Err(err) => return Err(err)
    };

    Ok(entity)
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

pub fn fast_serialize_dic<W: Write>(dictionary: &HashMap<&str, &str>, stream: &mut W) -> std::io::Result<()> {
    let mut writer = std::io::BufWriter::new(stream);

    // write header
    writer.write_all(&(dictionary.len() as u32).to_le_bytes())?;

    for (key, value) in dictionary {
        // write key
        writer.write_all(&(key.len() as u32).to_le_bytes())?;
        writer.write_all(key.as_bytes())?;

        // write value
        writer.write_all(&(value.len() as u32).to_le_bytes())?;
        writer.write_all(value.as_bytes())?;
    }

    writer.flush()?;

    Ok(())
}

pub fn fast_deserialize_dic(reader: &mut std::io::BufReader<&[u8]>) -> std::io::Result<HashMap<String, String>> {
    let mut dictionary = std::collections::HashMap::new();

    // let mut counter: [u8; 4] = [0; 4];
    {
        // reader.read(&mut counter).unwrap();
        let count = reverse_bytes(reader);

        for _ in 0..count {
            let key = extract_data(reader);
            let value = extract_data(reader);

            println!("key: {:?}, value: {:?}", key, value);
            
            dictionary.insert(key, value);
        }
    }

    Ok(dictionary)
}

fn extract_data(reader: &mut std::io::BufReader<&[u8]>) -> String {
    let mut buf_len= [0; 1];
    reader.read_exact(&mut buf_len).unwrap();
    let data_len = buf_len[0] as usize;

    let mut data_bytes = vec![0; data_len];
    reader.read_exact(&mut data_bytes).unwrap();

    String::from_utf8(data_bytes).unwrap()
}

fn reverse_bytes(reader: &mut std::io::BufReader<&[u8]>) -> u32 {
    // for i in 0..4 {
    //     counter.swap(0, 3 - i);
    // }
    let mut buf_len= [0; 4];
    reader.read_exact(&mut buf_len).unwrap();
    // buf_len[0] as u32
    u32::from_be_bytes(buf_len)
}

// fn read_exact_as_bytes<R: Read>(reader: &mut R, start:u8) -> std::io::Result<[u8; 4]> {
//     let mut buffer = [start; 4];
//     reader.read_exact(&mut buffer)?;
//     Ok(buffer)
// }

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
        password: String
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
        let deserialized_data: UserMock = deserilize_yaml(&mut serialized_data).unwrap();

        assert_eq!(deserialized_data.name, user.name);
        assert_eq!(deserialized_data.nested.id, user.nested.id);
    }

    #[test]
    fn bson_seralization() {
        let user = BsonUserMock { name: "ali".to_string(), password: "amir".to_string()};
        let serialized_user = serialize_bson(&user).unwrap();
        println!("the data is: {}", serialized_user);
        let user_back = deserilize_bson::<BsonUserMock>(serialized_user).unwrap();

        assert_eq!(user_back.name,"ali");
        assert_eq!(user_back.password,"amir");
    }
}