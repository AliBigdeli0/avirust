use std::collections::HashMap;

use base64::Engine;
use base64::engine::general_purpose;
use bson::{Binary, DateTime as BsonDateTime, doc, oid::ObjectId};
use chrono::prelude::*;
use chrono::{DateTime as ChronoDateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};

pub const WELL_COLLECTION_NAME: &str = "wells";
pub const START_DATE: i32 = 2000;

pub fn convert_to_bson_date(d: ChronoDateTime<Utc>) -> BsonDateTime {
    bson::DateTime::builder()
        .year(d.year())
        .month(d.month() as u8)
        .day(d.day() as u8)
        .hour(d.hour() as u8)
        .minute(d.minute() as u8)
        .second(d.second() as u8)
        .build()
        .unwrap()
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct DocumentValue<T = String> {
    #[serde(rename = "Document")]
    pub document: String,

    #[serde(rename = "Value")]
    pub value: T,
}

#[derive(Debug, Deserialize, Clone)]
pub enum SubTypeValues {
    AMQProducer = 1,
    AMQConsumer = 2,
    RabbitProducer = 4,
    RabbitConsumer = 8,
    AMQTextProducer = 16,
    AMQTextConsumer = 32,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct MQClientConfigure {
    #[serde(rename = "Uri")]
    pub uri: DocumentValue,

    #[serde(rename = "MQOfflineName")]
    pub mq_offline_name: DocumentValue,

    #[serde(rename = "SubType")]
    pub sub_type: DocumentValue<i32>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ServerConfigure {
    #[serde(rename = "MQClientConfigure")]
    pub mq_client_configure: MQClientConfigure,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Well {
    _id: ObjectId,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "IP")]
    pub ip: String,
    #[serde(rename = "IsActive")]
    pub is_active: bool,
    #[serde(rename = "ServerConfigure")]
    pub server_configure: ServerConfigure,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case, dead_code)]
pub struct TimeData {
    _id: ObjectId,

    #[serde(rename = "l")]
    pub data: Binary,

    #[serde(rename = "d")]
    pub date: BsonDateTime,
}

impl TimeData {
    pub fn new(d: ChronoDateTime<Utc>, data_str: String) -> Self {
        let data = general_purpose::STANDARD_NO_PAD.encode(data_str);

        let data_bytes = general_purpose::STANDARD_NO_PAD.decode(data).unwrap();

        let binary_data = Binary {
            subtype: bson::spec::BinarySubtype::Generic,
            bytes: data_bytes,
        };

        let bson_date = convert_to_bson_date(d);

        Self {
            _id: ObjectId::new(),
            data: binary_data,
            date: bson_date,
        }
    }

    pub fn new_binary(d: ChronoDateTime<Utc>, binary_data: Binary) -> Self {
        let bson_date = convert_to_bson_date(d);

        Self {
            _id: ObjectId::new(),
            data: binary_data,
            date: bson_date,
        }
    }

    pub fn new_hash(d: ChronoDateTime<Utc>, hash: &HashMap<String, String>) -> Self {
        let bson_date = convert_to_bson_date(d);
        let bson_binary = TimeData::hash_to_binary(hash).unwrap();
        Self {
            _id: ObjectId::new(),
            date: bson_date,
            data: bson_binary,
        }
    }

    pub fn generate_collection_name(id: &str, date: ChronoDateTime<Utc>) -> String {
        let year = date.year() - START_DATE;
        let month = date.month();
        let day = date.day();
        format!("c_{id}_{year:02}_{month:02}_{day:02}")
    }

    pub fn data_to_hash(binary: Binary) -> std::io::Result<HashMap<String, String>> {
        super::super::serializer_helper::fast_deserialize_dic(&binary.bytes)
    }

    pub fn hash_to_binary(hash: &HashMap<String, String>) -> std::io::Result<Binary> {
        let serlialized_hash = super::super::serializer_helper::fast_serialize_dic(&hash)?;
        let encoded_data = general_purpose::STANDARD.encode(serlialized_hash);

        let ret_binary = match Binary::from_base64(encoded_data, None) {
            Ok(binary) => binary,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "error in convert hash to binary!!!",
                ));
            }
        };

        Ok(ret_binary)
    }
}

#[cfg(test)]
mod test_db {
    use super::*;
    use bson::spec::BinarySubtype;
    use bson::{Bson, Document};
    use serde::de::DeserializeOwned;
    use std::collections::HashSet;

    fn datetime(year: i32, month: u32, day: u32) -> ChronoDateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 12, 34, 56)
            .single()
            .unwrap()
    }

    fn sample_date() -> ChronoDateTime<Utc> {
        datetime(2024, 2, 29)
    }

    fn binary(bytes: &[u8]) -> Binary {
        Binary {
            subtype: BinarySubtype::Generic,
            bytes: bytes.to_vec(),
        }
    }

    fn sample_hash() -> HashMap<String, String> {
        HashMap::from([
            ("pressure".into(), "123.45".into()),
            ("temperature".into(), "-12.5".into()),
            ("status".into(), "active".into()),
        ])
    }

    fn document_value() -> Document {
        doc! { "Document": "Connection URI", "Value": "amqp://localhost" }
    }

    fn mq_config() -> Document {
        doc! {
            "Uri": document_value(),
            "MQOfflineName": { "Document": "Offline queue", "Value": "offline-well-1" },
        }
    }

    fn server_config() -> Document {
        doc! { "MQClientConfigure": mq_config() }
    }

    fn well_document() -> Document {
        doc! {
            "_id": ObjectId::parse_str("507f1f77bcf86cd799439011").unwrap(),
            "Name": "Well 1",
            "ID": "well-001",
            "IP": "127.0.0.1",
            "IsActive": true,
            "ServerConfigure": server_config(),
        }
    }

    fn time_data_document() -> Document {
        doc! {
            "_id": ObjectId::parse_str("507f1f77bcf86cd799439012").unwrap(),
            "l": binary(&[0, 1, 127, 128, 255]),
            "d": BsonDateTime::from_millis(1_709_210_096_789),
        }
    }

    fn deserialize<T: DeserializeOwned>(document: Document) -> T {
        bson::deserialize_from_document(document).unwrap()
    }

    fn assert_required_fields<T: DeserializeOwned>(document: &Document, fields: &[&str]) {
        for field in fields {
            let mut missing = document.clone();
            missing.remove(*field);
            assert!(
                bson::deserialize_from_document::<T>(missing).is_err(),
                "missing {field} must be rejected"
            );
            let mut null = document.clone();
            null.insert(*field, Bson::Null);
            assert!(
                bson::deserialize_from_document::<T>(null).is_err(),
                "null {field} must be rejected"
            );
        }
    }

    fn assert_wrong_types<T: DeserializeOwned>(document: &Document, fields: &[(&str, Bson)]) {
        for (field, value) in fields {
            let mut invalid = document.clone();
            invalid.insert(*field, value.clone());
            assert!(
                bson::deserialize_from_document::<T>(invalid).is_err(),
                "invalid type for {field} must be rejected"
            );
        }
    }

    fn assert_same_time_data(actual: &TimeData, expected: &TimeData) {
        assert_eq!(actual._id, expected._id);
        assert_eq!(actual.data, expected.data);
        assert_eq!(actual.date, expected.date);
    }

    #[test]
    fn bson_date_preserves_calendar_and_clock_components() {
        for date in [
            sample_date(),
            Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap(),
            datetime(1900, 3, 1),
            datetime(2000, 2, 29),
        ] {
            assert_eq!(convert_to_bson_date(date).to_chrono(), date);
        }
    }

    #[test]
    fn bson_date_handles_unix_epoch() {
        let epoch = Utc.timestamp_opt(0, 0).unwrap();
        assert_eq!(convert_to_bson_date(epoch).timestamp_millis(), 0);
    }

    #[test]
    fn bson_date_handles_dates_before_unix_epoch() {
        for seconds in [-1, -86_400, -2_208_988_800] {
            let date = Utc.timestamp_opt(seconds, 0).unwrap();
            assert_eq!(
                convert_to_bson_date(date).timestamp_millis(),
                seconds * 1_000
            );
        }
    }

    #[test]
    fn bson_date_discards_subsecond_precision_without_rounding() {
        for nanos in [1, 999_999, 1_000_000, 123_456_789, 999_999_999] {
            let date = sample_date().with_nanosecond(nanos).unwrap();
            assert_eq!(convert_to_bson_date(date).to_chrono(), sample_date());
        }
    }

    #[test]
    fn bson_date_discards_subseconds_before_unix_epoch() {
        let date = Utc.timestamp_opt(-1, 999_999_999).unwrap();
        assert_eq!(convert_to_bson_date(date).timestamp_millis(), -1_000);
    }

    #[test]
    fn bson_date_uses_utc_after_offset_conversion() {
        let date = ChronoDateTime::parse_from_rfc3339("2024-03-01T01:30:45+03:00")
            .unwrap()
            .with_timezone(&Utc);
        let expected = Utc.with_ymd_and_hms(2024, 2, 29, 22, 30, 45).unwrap();
        assert_eq!(convert_to_bson_date(date).to_chrono(), expected);
    }

    #[test]
    fn collection_name_zero_pads_year_month_and_day() {
        for (year, month, day, expected) in [
            (2000, 1, 1, "c_tt_00_01_01"),
            (2009, 9, 9, "c_tt_09_09_09"),
            (2023, 3, 4, "c_tt_23_03_04"),
            (2023, 10, 30, "c_tt_23_10_30"),
            (2024, 2, 29, "c_tt_24_02_29"),
            (2099, 12, 31, "c_tt_99_12_31"),
        ] {
            assert_eq!(
                TimeData::generate_collection_name("tt", datetime(year, month, day)),
                expected
            );
        }
    }

    #[test]
    fn collection_name_handles_years_outside_two_digit_range() {
        for (year, expected) in [
            (1999, "c_tt_-1_01_01"),
            (1900, "c_tt_-100_01_01"),
            (2100, "c_tt_100_01_01"),
        ] {
            assert_eq!(
                TimeData::generate_collection_name("tt", datetime(year, 1, 1)),
                expected
            );
        }
    }

    #[test]
    fn collection_name_preserves_identifier_verbatim() {
        for id in ["", "well_001", "a-b.c", "two words", "井戸", "a/b", " a "] {
            assert_eq!(
                TimeData::generate_collection_name(id, sample_date()),
                format!("c_{id}_24_02_29")
            );
        }
    }

    #[test]
    fn collection_name_ignores_time_of_day_and_subseconds() {
        for (hour, minute, second, nanos) in [(0, 0, 0, 0), (23, 59, 59, 999_999_999)] {
            let date = Utc
                .with_ymd_and_hms(2024, 2, 29, hour, minute, second)
                .unwrap()
                .with_nanosecond(nanos)
                .unwrap();
            assert_eq!(
                TimeData::generate_collection_name("well", date),
                "c_well_24_02_29"
            );
        }
    }

    #[test]
    fn collection_name_changes_at_calendar_boundaries() {
        for (before, after, first, second) in [
            (
                datetime(2023, 12, 31),
                datetime(2024, 1, 1),
                "c_w_23_12_31",
                "c_w_24_01_01",
            ),
            (
                datetime(2024, 2, 29),
                datetime(2024, 3, 1),
                "c_w_24_02_29",
                "c_w_24_03_01",
            ),
        ] {
            assert_eq!(TimeData::generate_collection_name("w", before), first);
            assert_eq!(TimeData::generate_collection_name("w", after), second);
        }
    }

    #[test]
    fn collection_name_uses_utc_calendar_date() {
        let date = ChronoDateTime::parse_from_rfc3339("2024-01-01T00:30:00+01:00")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(
            TimeData::generate_collection_name("w", date),
            "c_w_23_12_31"
        );
    }

    #[test]
    fn new_stores_exact_utf8_bytes_as_generic_binary() {
        // Include every base64 padding remainder and strings that look like base64.
        for input in [
            "",
            "a",
            "ab",
            "abc",
            "abcd",
            "data",
            "ZGF0YQ==",
            "井戸 🛢",
            "a\0b\n\r\t",
        ] {
            let data = TimeData::new(sample_date(), input.to_owned());
            assert_eq!(data.data, binary(input.as_bytes()), "input {input:?}");
            assert_eq!(data.date.to_chrono(), sample_date());
        }
    }

    #[test]
    fn new_preserves_large_strings() {
        let input = "sensor:井戸\n".repeat(10_000);
        let data = TimeData::new(sample_date(), input.clone());
        assert_eq!(data.data.bytes, input.as_bytes());
    }

    #[test]
    fn new_binary_preserves_all_byte_values() {
        let input = Binary {
            subtype: BinarySubtype::Generic,
            bytes: (0..=255).collect(),
        };
        let data = TimeData::new_binary(sample_date(), input.clone());
        assert_eq!(data.data, input);
        assert_eq!(data.date.to_chrono(), sample_date());
    }

    #[test]
    fn new_binary_preserves_subtype() {
        for subtype in [
            BinarySubtype::Generic,
            BinarySubtype::Function,
            BinarySubtype::BinaryOld,
            BinarySubtype::Uuid,
            BinarySubtype::Md5,
            BinarySubtype::UserDefined(0x80),
        ] {
            let input = Binary {
                subtype,
                bytes: vec![42; 16],
            };
            let data = TimeData::new_binary(sample_date(), input.clone());
            assert_eq!(data.data, input);
        }
    }

    #[test]
    fn new_binary_accepts_empty_payload() {
        let data = TimeData::new_binary(sample_date(), binary(&[]));
        assert_eq!(data.data, binary(&[]));
    }

    #[test]
    fn new_hash_stores_recoverable_dictionary() {
        let hash = sample_hash();
        let data = TimeData::new_hash(sample_date(), &hash);
        assert_eq!(data.date.to_chrono(), sample_date());
        assert_eq!(data.data.subtype, BinarySubtype::Generic);
        assert_eq!(TimeData::data_to_hash(data.data).unwrap(), hash);
    }

    #[test]
    fn new_hash_accepts_empty_dictionary() {
        let data = TimeData::new_hash(sample_date(), &HashMap::new());
        assert_eq!(data.data, binary(&[0, 0, 0, 0]));
        assert!(TimeData::data_to_hash(data.data).unwrap().is_empty());
    }

    #[test]
    fn new_hash_takes_independent_snapshot() {
        let mut hash = sample_hash();
        let original = hash.clone();
        let data = TimeData::new_hash(sample_date(), &hash);
        assert_eq!(hash, original);
        hash.insert("pressure".into(), "999".into());
        hash.clear();
        assert_eq!(TimeData::data_to_hash(data.data).unwrap(), original);
    }

    #[test]
    fn all_constructors_discard_subseconds() {
        let date = sample_date().with_nanosecond(987_654_321).unwrap();
        for data in [
            TimeData::new(date, "abc".into()),
            TimeData::new_binary(date, binary(&[1, 2, 3])),
            TimeData::new_hash(date, &sample_hash()),
        ] {
            assert_eq!(data.date.to_chrono(), sample_date());
        }
    }

    #[test]
    fn all_constructors_support_dates_before_unix_epoch() {
        let date = Utc.timestamp_opt(-1, 0).unwrap();
        for data in [
            TimeData::new(date, String::new()),
            TimeData::new_binary(date, binary(&[])),
            TimeData::new_hash(date, &HashMap::new()),
        ] {
            assert_eq!(data.date.timestamp_millis(), -1_000);
        }
    }

    #[test]
    fn all_constructors_generate_distinct_object_ids() {
        let mut ids = HashSet::new();
        for _ in 0..64 {
            for data in [
                TimeData::new(sample_date(), "same payload".into()),
                TimeData::new_binary(sample_date(), binary(&[1])),
                TimeData::new_hash(sample_date(), &sample_hash()),
            ] {
                assert_ne!(data._id, ObjectId::from_bytes([0; 12]));
                assert!(ids.insert(data._id), "constructor reused an ObjectId");
            }
        }
    }

    #[test]
    fn hash_to_binary_uses_expected_wire_format() {
        let hash = HashMap::from([("a".into(), "bc".into())]);
        // Little-endian entry count, then byte-sized key/value lengths and UTF-8.
        assert_eq!(
            TimeData::hash_to_binary(&hash).unwrap(),
            binary(&[1, 0, 0, 0, 1, b'a', 2, b'b', b'c'])
        );
    }

    #[test]
    fn empty_hash_serializes_to_zero_entry_header() {
        assert_eq!(
            TimeData::hash_to_binary(&HashMap::new()).unwrap(),
            binary(&[0, 0, 0, 0])
        );
    }

    #[test]
    fn hash_round_trip_preserves_all_entries() {
        let hash = sample_hash();
        let data = TimeData::hash_to_binary(&hash).unwrap();
        assert_eq!(data.subtype, BinarySubtype::Generic);
        assert_eq!(TimeData::data_to_hash(data).unwrap(), hash);
    }

    #[test]
    fn hash_round_trip_preserves_empty_keys_and_values() {
        for hash in [
            HashMap::from([("".into(), "".into())]),
            HashMap::from([("".into(), "value".into()), ("key".into(), "".into())]),
        ] {
            let data = TimeData::hash_to_binary(&hash).unwrap();
            assert_eq!(TimeData::data_to_hash(data).unwrap(), hash);
        }
    }

    #[test]
    fn hash_round_trip_preserves_unicode_and_control_characters() {
        let hash = HashMap::from([
            ("井戸 🛢".into(), "فشار ۱۲۳".into()),
            ("a\0b".into(), "line 1\nline 2\r\t\0".into()),
            ("quotes\"\\".into(), "{json}: [yaml], = value".into()),
        ]);
        assert_eq!(
            TimeData::data_to_hash(TimeData::hash_to_binary(&hash).unwrap()).unwrap(),
            hash
        );
    }

    #[test]
    fn hash_round_trip_supports_every_single_byte_string_length() {
        // Exercises the signed-byte boundary as well as the maximum byte length.
        for length in 0..=255 {
            let hash = HashMap::from([("k".repeat(length), "v".repeat(255 - length))]);
            let data = TimeData::hash_to_binary(&hash).unwrap();
            assert_eq!(
                TimeData::data_to_hash(data).unwrap(),
                hash,
                "length {length}"
            );
        }
    }

    #[test]
    fn hash_to_binary_accepts_every_base64_padding_remainder() {
        for value in ["", "a", "ab", "abc", "abcd", "abcde"] {
            let hash = HashMap::from([("k".into(), value.to_owned())]);
            let data = TimeData::hash_to_binary(&hash).unwrap();
            assert_eq!(data.bytes.len(), 7 + value.len());
            assert_eq!(TimeData::data_to_hash(data).unwrap(), hash);
        }
    }

    #[test]
    fn hash_round_trip_supports_maximum_utf8_byte_length() {
        let hash = HashMap::from([("井".repeat(85), "é".repeat(127) + "!")]);
        let data = TimeData::hash_to_binary(&hash).unwrap();
        assert_eq!(TimeData::data_to_hash(data).unwrap(), hash);
    }

    #[test]
    fn hash_round_trip_supports_entry_counts_larger_than_one_byte() {
        let hash: HashMap<_, _> = (0..512)
            .map(|i| (format!("key-{i}"), format!("value-{i}")))
            .collect();
        let data = TimeData::hash_to_binary(&hash).unwrap();
        assert_eq!(&data.bytes[..4], &[0, 2, 0, 0]);
        assert_eq!(TimeData::data_to_hash(data).unwrap(), hash);
    }

    #[test]
    fn hash_to_binary_does_not_mutate_input() {
        let hash = sample_hash();
        let original = hash.clone();
        TimeData::hash_to_binary(&hash).unwrap();
        assert_eq!(hash, original);
    }

    #[test]
    fn data_to_hash_decodes_known_wire_payload() {
        let data = binary(&[2, 0, 0, 0, 1, b'a', 1, b'1', 1, b'b', 1, b'2']);
        assert_eq!(
            TimeData::data_to_hash(data).unwrap(),
            HashMap::from([("a".into(), "1".into()), ("b".into(), "2".into())])
        );
    }

    #[test]
    fn data_to_hash_decodes_zero_entry_header() {
        assert!(
            TimeData::data_to_hash(binary(&[0, 0, 0, 0]))
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn data_to_hash_uses_last_value_for_duplicate_keys() {
        let data = binary(&[2, 0, 0, 0, 1, b'a', 1, b'1', 1, b'a', 1, b'2']);
        assert_eq!(
            TimeData::data_to_hash(data).unwrap(),
            HashMap::from([("a".into(), "2".into())])
        );
    }

    #[test]
    fn data_to_hash_ignores_trailing_bytes() {
        let data = binary(&[1, 0, 0, 0, 1, b'a', 1, b'b', 255, 0, 42]);
        assert_eq!(
            TimeData::data_to_hash(data).unwrap(),
            HashMap::from([("a".into(), "b".into())])
        );
    }

    #[test]
    fn data_to_hash_reads_bytes_independently_of_binary_subtype() {
        let mut data = TimeData::hash_to_binary(&sample_hash()).unwrap();
        data.subtype = BinarySubtype::UserDefined(0x80);
        assert_eq!(TimeData::data_to_hash(data).unwrap(), sample_hash());
    }

    // The underlying decoder currently unwraps read/UTF-8 errors despite returning
    // io::Result. These tests document its panic behavior for malformed payloads.
    #[test]
    #[should_panic(expected = "UnexpectedEof")]
    fn data_to_hash_panics_on_empty_payload() {
        let _ = TimeData::data_to_hash(binary(&[]));
    }

    #[test]
    fn data_to_hash_panics_at_every_truncated_payload_boundary() {
        let bytes = [
            1, 0, 0, 0, 3, b'k', b'e', b'y', 5, b'v', b'a', b'l', b'u', b'e',
        ];
        for length in 1..bytes.len() {
            assert!(
                std::panic::catch_unwind(|| TimeData::data_to_hash(binary(&bytes[..length])))
                    .is_err(),
                "truncated payload of length {length} must panic"
            );
        }
    }

    #[test]
    #[should_panic(expected = "UnexpectedEof")]
    fn data_to_hash_panics_when_count_exceeds_available_entries() {
        let _ = TimeData::data_to_hash(binary(&[2, 0, 0, 0, 1, b'a', 1, b'b']));
    }

    #[test]
    #[should_panic(expected = "FromUtf8Error")]
    fn data_to_hash_panics_on_invalid_utf8_key() {
        let _ = TimeData::data_to_hash(binary(&[1, 0, 0, 0, 1, 255, 1, b'v']));
    }

    #[test]
    #[should_panic(expected = "FromUtf8Error")]
    fn data_to_hash_panics_on_invalid_utf8_value() {
        let _ = TimeData::data_to_hash(binary(&[1, 0, 0, 0, 1, b'k', 1, 255]));
    }

    #[test]
    fn document_value_deserializes_renamed_fields() {
        let value: DocumentValue = deserialize(document_value());
        assert_eq!(value.document, "Connection URI");
        assert_eq!(value.value, "amqp://localhost");
    }

    #[test]
    fn document_value_accepts_empty_and_unicode_strings() {
        let value: DocumentValue = deserialize(doc! { "Document": "", "Value": "井戸 🛢" });
        assert_eq!(value.document, "");
        assert_eq!(value.value, "井戸 🛢");
    }

    #[test]
    fn document_value_requires_both_fields() {
        assert_required_fields::<DocumentValue>(&document_value(), &["Document", "Value"]);
    }

    #[test]
    fn document_value_rejects_non_string_fields() {
        assert_wrong_types::<DocumentValue>(
            &document_value(),
            &[("Document", Bson::Int32(1)), ("Value", Bson::Boolean(true))],
        );
    }

    #[test]
    fn document_value_rejects_rust_field_names() {
        assert!(
            bson::deserialize_from_document::<DocumentValue>(
                doc! { "document": "description", "value": "value" }
            )
            .is_err()
        );
    }

    #[test]
    fn document_value_ignores_unknown_fields() {
        let mut document = document_value();
        document.insert("FutureOption", doc! { "nested": true });
        let value: DocumentValue = deserialize(document);
        assert_eq!(value.value, "amqp://localhost");
    }

    #[test]
    fn document_value_clone_owns_independent_strings() {
        let original: DocumentValue = deserialize(document_value());
        let mut cloned = original.clone();
        cloned.document.push_str(" changed");
        cloned.value.clear();
        assert_eq!(original.document, "Connection URI");
        assert_eq!(original.value, "amqp://localhost");
    }

    #[test]
    fn mq_config_deserializes_nested_values() {
        let config: MQClientConfigure = deserialize(mq_config());
        assert_eq!(config.uri.document, "Connection URI");
        assert_eq!(config.uri.value, "amqp://localhost");
        assert_eq!(config.mq_offline_name.document, "Offline queue");
        assert_eq!(config.mq_offline_name.value, "offline-well-1");
    }

    #[test]
    fn mq_config_requires_both_fields() {
        assert_required_fields::<MQClientConfigure>(&mq_config(), &["Uri", "MQOfflineName"]);
    }

    #[test]
    fn mq_config_rejects_wrong_types_and_incomplete_nested_values() {
        for field in ["Uri", "MQOfflineName"] {
            assert_wrong_types::<MQClientConfigure>(
                &mq_config(),
                &[
                    (field, Bson::String("invalid".into())),
                    (field, Bson::Document(doc! { "Value": "x" })),
                ],
            );
        }
    }

    #[test]
    fn mq_config_rejects_rust_field_names() {
        assert!(
            bson::deserialize_from_document::<MQClientConfigure>(doc! {
                "uri": document_value(), "mq_offline_name": document_value()
            })
            .is_err()
        );
    }

    #[test]
    fn server_config_deserializes_nested_mq_config() {
        let config: ServerConfigure = deserialize(server_config());
        assert_eq!(config.mq_client_configure.uri.value, "amqp://localhost");
        assert_eq!(
            config.mq_client_configure.mq_offline_name.value,
            "offline-well-1"
        );
    }

    #[test]
    fn server_config_requires_mq_config() {
        assert_required_fields::<ServerConfigure>(&server_config(), &["MQClientConfigure"]);
    }

    #[test]
    fn server_config_rejects_wrong_type_and_incomplete_mq_config() {
        assert_wrong_types::<ServerConfigure>(
            &server_config(),
            &[
                ("MQClientConfigure", Bson::String("invalid".into())),
                ("MQClientConfigure", Bson::Document(doc! {})),
            ],
        );
    }

    #[test]
    fn server_config_rejects_rust_field_name() {
        assert!(
            bson::deserialize_from_document::<ServerConfigure>(doc! {
                "mq_client_configure": mq_config()
            })
            .is_err()
        );
    }

    #[test]
    fn well_deserializes_all_fields_and_nested_config() {
        let well: Well = deserialize(well_document());
        assert_eq!(well._id.to_hex(), "507f1f77bcf86cd799439011");
        assert_eq!(well.name, "Well 1");
        assert_eq!(well.id, "well-001");
        assert_eq!(well.ip, "127.0.0.1");
        assert!(well.is_active);
        let config = well.server_configure.mq_client_configure;
        assert_eq!(config.uri.document, "Connection URI");
        assert_eq!(config.uri.value, "amqp://localhost");
        assert_eq!(config.mq_offline_name.document, "Offline queue");
        assert_eq!(config.mq_offline_name.value, "offline-well-1");
    }

    #[test]
    fn well_accepts_inactive_state() {
        let mut document = well_document();
        document.insert("IsActive", false);
        assert!(!deserialize::<Well>(document).is_active);
    }

    #[test]
    fn well_accepts_empty_strings_without_application_validation() {
        let mut document = well_document();
        for field in ["Name", "ID", "IP"] {
            document.insert(field, "");
        }
        let well: Well = deserialize(document);
        assert_eq!(well.name, "");
        assert_eq!(well.id, "");
        assert_eq!(well.ip, "");
    }

    #[test]
    fn well_requires_all_fields() {
        assert_required_fields::<Well>(
            &well_document(),
            &["_id", "Name", "ID", "IP", "IsActive", "ServerConfigure"],
        );
    }

    #[test]
    fn well_rejects_wrong_field_types() {
        assert_wrong_types::<Well>(
            &well_document(),
            &[
                ("_id", Bson::Int32(1)),
                ("Name", Bson::Boolean(true)),
                ("ID", Bson::Int32(42)),
                ("IP", Bson::Array(vec![])),
                ("IsActive", Bson::String("true".into())),
                ("IsActive", Bson::Int32(1)),
                ("ServerConfigure", Bson::String("invalid".into())),
            ],
        );
    }

    #[test]
    fn well_rejects_malformed_object_id() {
        assert_wrong_types::<Well>(
            &well_document(),
            &[("_id", Bson::String("not-an-object-id".into()))],
        );
    }

    #[test]
    fn well_rejects_rust_field_names() {
        for (wire, rust) in [
            ("Name", "name"),
            ("ID", "id"),
            ("IP", "ip"),
            ("IsActive", "is_active"),
            ("ServerConfigure", "server_configure"),
        ] {
            let mut document = well_document();
            let value = document.remove(wire).unwrap();
            document.insert(rust, value);
            assert!(
                bson::deserialize_from_document::<Well>(document).is_err(),
                "{rust}"
            );
        }
    }

    #[test]
    fn well_rejects_incomplete_configuration_at_each_level() {
        for configuration in [
            doc! {},
            doc! { "MQClientConfigure": {} },
            doc! { "MQClientConfigure": { "Uri": document_value() } },
            doc! { "MQClientConfigure": {
                "Uri": { "Document": "missing value" },
                "MQOfflineName": document_value(),
            } },
        ] {
            let mut document = well_document();
            document.insert("ServerConfigure", configuration);
            assert!(bson::deserialize_from_document::<Well>(document).is_err());
        }
    }

    #[test]
    fn well_ignores_unknown_fields_at_every_nesting_level() {
        let mut uri = document_value();
        uri.insert("Extra", true);
        let mut mq = mq_config();
        mq.insert("Uri", uri);
        mq.insert("Extra", 42);
        let mut server = server_config();
        server.insert("MQClientConfigure", mq);
        server.insert("Extra", "extra");
        let mut document = well_document();
        document.insert("ServerConfigure", server);
        document.insert("Extra", Bson::Null);
        let well: Well = deserialize(document);
        assert_eq!(well.name, "Well 1");
        assert_eq!(
            well.server_configure.mq_client_configure.uri.value,
            "amqp://localhost"
        );
    }

    #[test]
    fn well_clone_preserves_id_and_owns_independent_nested_fields() {
        let original: Well = deserialize(well_document());
        let mut cloned = original.clone();
        assert_eq!(cloned._id, original._id);
        assert_eq!(cloned.name, original.name);
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.ip, original.ip);
        assert_eq!(cloned.is_active, original.is_active);
        cloned.name.clear();
        cloned.id.clear();
        cloned.ip.clear();
        cloned.is_active = false;
        cloned
            .server_configure
            .mq_client_configure
            .uri
            .value
            .clear();
        cloned
            .server_configure
            .mq_client_configure
            .mq_offline_name
            .document
            .clear();
        assert_eq!(original.name, "Well 1");
        assert_eq!(original.id, "well-001");
        assert_eq!(original.ip, "127.0.0.1");
        assert!(original.is_active);
        assert_eq!(
            original.server_configure.mq_client_configure.uri.value,
            "amqp://localhost"
        );
        assert_eq!(
            original
                .server_configure
                .mq_client_configure
                .mq_offline_name
                .document,
            "Offline queue"
        );
    }

    #[test]
    fn time_data_serializes_exact_database_field_names_and_types() {
        let data = TimeData::new(sample_date(), "payload".into());
        let document = bson::serialize_to_document(&data).unwrap();
        assert_eq!(document.len(), 3);
        assert_eq!(document.get_object_id("_id").unwrap(), data._id);
        assert_eq!(document.get("l"), Some(&Bson::Binary(binary(b"payload"))));
        assert_eq!(document.get_datetime("d").unwrap(), &data.date);
        assert!(!document.contains_key("data"));
        assert!(!document.contains_key("date"));
    }

    #[test]
    fn time_data_deserializes_existing_document_without_regenerating_id() {
        let document = time_data_document();
        let data: TimeData = deserialize(document.clone());
        assert_eq!(data._id, document.get_object_id("_id").unwrap());
        assert_eq!(data.data, binary(&[0, 1, 127, 128, 255]));
        assert_eq!(data.date.timestamp_millis(), 1_709_210_096_789);
    }

    #[test]
    fn time_data_document_round_trip_preserves_all_constructor_outputs() {
        for original in [
            TimeData::new(sample_date(), "井戸".into()),
            TimeData::new_binary(
                sample_date(),
                Binary {
                    subtype: BinarySubtype::UserDefined(0x80),
                    bytes: vec![0, 255, 128],
                },
            ),
            TimeData::new_hash(sample_date(), &sample_hash()),
        ] {
            let document = bson::serialize_to_document(&original).unwrap();
            let restored: TimeData = deserialize(document);
            assert_same_time_data(&restored, &original);
        }
    }

    #[test]
    fn time_data_raw_bson_round_trip_preserves_binary_and_milliseconds() {
        let original: TimeData = deserialize(time_data_document());
        let bytes = bson::serialize_to_vec(&original).unwrap();
        let restored: TimeData = bson::deserialize_from_slice(&bytes).unwrap();
        assert_same_time_data(&restored, &original);
    }

    #[test]
    fn time_data_deserializes_canonical_extended_json() {
        let json = r#"{
            "_id": {"$oid": "507f1f77bcf86cd799439012"},
            "l": {"$binary": {"base64": "AAF/gP8=", "subType": "00"}},
            "d": {"$date": {"$numberLong": "1709210096789"}}
        }"#;
        let restored: TimeData = serde_json::from_str(json).unwrap();
        let expected: TimeData = deserialize(time_data_document());
        assert_same_time_data(&restored, &expected);
    }

    #[test]
    fn time_data_json_round_trip_preserves_custom_binary_subtype() {
        let original = TimeData::new_binary(
            sample_date(),
            Binary {
                subtype: BinarySubtype::UserDefined(0x80),
                bytes: vec![0, 1, 127, 128, 255],
            },
        );
        let json = serde_json::to_string(&original).unwrap();
        let restored: TimeData = serde_json::from_str(&json).unwrap();
        assert_same_time_data(&restored, &original);
    }

    #[test]
    fn time_data_json_serializes_generic_binary_as_byte_array() {
        let original: TimeData = deserialize(time_data_document());
        let json = serde_json::to_value(&original).unwrap();
        assert_eq!(json["_id"]["$oid"], "507f1f77bcf86cd799439012");
        assert_eq!(json["l"], serde_json::json!([0, 1, 127, 128, 255]));
        assert!(json.get("d").is_some());
        assert!(json.get("data").is_none());
        assert!(json.get("date").is_none());
        // BSON's Generic subtype serializes as bytes, which JSON renders as an
        // array. Deserialization needs Extended JSON's $binary representation.
        assert!(serde_json::from_value::<TimeData>(json).is_err());
    }

    #[test]
    fn time_data_round_trip_preserves_empty_binary() {
        let original = TimeData::new_binary(sample_date(), binary(&[]));
        let document = bson::serialize_to_document(&original).unwrap();
        let restored: TimeData = deserialize(document);
        assert_same_time_data(&restored, &original);
    }

    #[test]
    fn time_data_requires_all_database_fields() {
        assert_required_fields::<TimeData>(&time_data_document(), &["_id", "l", "d"]);
    }

    #[test]
    fn time_data_rejects_wrong_database_field_types() {
        assert_wrong_types::<TimeData>(
            &time_data_document(),
            &[
                ("_id", Bson::Int32(1)),
                ("l", Bson::String("payload".into())),
                ("l", Bson::Int32(1)),
                ("d", Bson::String("2024-02-29".into())),
                ("d", Bson::Boolean(false)),
            ],
        );
    }

    #[test]
    fn time_data_rejects_rust_field_names() {
        for (wire, rust) in [("l", "data"), ("d", "date")] {
            let mut document = time_data_document();
            let value = document.remove(wire).unwrap();
            document.insert(rust, value);
            assert!(bson::deserialize_from_document::<TimeData>(document).is_err());
        }
    }

    #[test]
    fn time_data_ignores_unknown_database_fields() {
        let original: TimeData = deserialize(time_data_document());
        let mut document = time_data_document();
        document.insert("future_field", "value");
        let restored: TimeData = deserialize(document);
        assert_same_time_data(&restored, &original);
    }

    #[test]
    fn time_data_hash_survives_raw_bson_round_trip() {
        let hash = sample_hash();
        let original = TimeData::new_hash(sample_date(), &hash);
        let bytes = bson::serialize_to_vec(&original).unwrap();
        let restored: TimeData = bson::deserialize_from_slice(&bytes).unwrap();
        assert_same_time_data(&restored, &original);
        assert_eq!(TimeData::data_to_hash(restored.data).unwrap(), hash);
    }
}
