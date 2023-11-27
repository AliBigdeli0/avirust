use serde::Deserialize;
use serde::Serialize;
pub use serde_json::{from_str, to_string};

pub fn serialize_json<T>(obj:&T) -> Result<String, serde_json::Error> where
    T: ?Sized + Serialize,
{
    let string_item = to_string(obj)?;
    Ok(string_item)
}

pub fn deserilize_json<'a,T>(entity_str:&'a str) -> Result<T, serde_json::Error>  where T: Deserialize<'a>,
{
    let deser:T = from_str(entity_str)?;
    Ok(deser)
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct UserMock<'a> {
        name: &'a str,
        password: &'a str,
        age: i32
    }

    #[test]
    fn test_serialize_to_string() {
        let user = UserMock { name: "ali", password: "amir", age : 10};
        let serilizad_data = serialize_json(&user).unwrap();
        assert!(serilizad_data.contains("ali"));
        assert!(serilizad_data.contains("amir"));
        assert!(serilizad_data.contains("10"));
    }

    #[test]
    fn test_deserialize_from_string() {
        let user = UserMock { name: "hani", password: "madar", age : 20};

        let serialized_data = serialize_json(&user).unwrap();
        let deserliazlied_user: UserMock = deserilize_json(&serialized_data).unwrap();

        assert_eq!(deserliazlied_user.name, "hani");
        assert_eq!(deserliazlied_user.password, "madar");
        assert_eq!(deserliazlied_user.age, 20);
    }
}