use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub fn hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod test_hash {
    use super::*;

    #[test]
    fn convert_to_hash() {
        let value = String::from("value");
        assert_ne!(hash(&value), 0)
    }
}
