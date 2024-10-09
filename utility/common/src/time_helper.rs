use std::path::Path;

use chrono::{
    format::{DelayedFormat, StrftimeItems},
    DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc,
};

const BSON_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";
const EXCEL_TIME_FORMAT: &str = "%m/%d/%Y %H:%M:%S";
const FILENAME_DATE_FORMAT: &str = "%y_%m_%d";

pub fn get_now_utc() -> DateTime<Utc> {
    Utc::now()
}

pub fn get_bson_time<'a>(dt: DateTime<Utc>) -> DelayedFormat<StrftimeItems<'a>> {
    dt.format(BSON_TIME_FORMAT)
}

pub fn get_bson_time_str<'a>(dt: DateTime<Utc>) -> String {
    get_bson_time(dt).to_string()
}

fn create_datetime(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> NaiveDateTime {
    let naive_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let naive_time = NaiveTime::from_hms_opt(hour, minute, second).unwrap();

    NaiveDateTime::new(naive_date, naive_time)
}

pub fn create_datetime_utc(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> DateTime<Utc> {
    DateTime::from_naive_utc_and_offset(
        create_datetime(year, month, day, hour, minute, second),
        Utc,
    )
}

pub fn convert_string_to_local(str_date: impl AsRef<str>) -> Result<DateTime<Local>, String> {
    let parser_date_result = NaiveDateTime::parse_from_str(str_date.as_ref(), EXCEL_TIME_FORMAT);

    if let Ok(parsed_date) = parser_date_result {
        if let Some(local_date_result) = parsed_date.and_local_timezone(Local).single() {
            Ok(local_date_result)
        } else {
            Err(String::from("error in date time conversion"))
        }
    } else {
        Err(String::from("Error in convert nativeDateTime to str"))
    }
}

pub fn convert_local_to_utc(datetime: DateTime<Local>) -> DateTime<Utc> {
    datetime.with_timezone(&Utc)
}

pub fn convert_naive_to_local(
    naive_opt: Option<NaiveDateTime>,
) -> Result<DateTime<Local>, Box<dyn std::error::Error>> {
    if let Some(naive_date_time) = naive_opt {
        match naive_date_time.and_local_timezone(Local).single() {
            Some(s) => Ok(s),
            None => Err("can not convert to local datetime".into()),
        }
    } else {
        Err("naive data is empty".into())
    }
}

pub fn remove_extension(file_name: &str) -> &str {
    let path = Path::new(file_name);
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(file_name)
}

pub fn extract_date_from_xml_file(
    file_name: &str,
) -> Result<NaiveDate, Box<dyn std::error::Error>> {
    let removed_extension_file = remove_extension(file_name);
    if removed_extension_file.starts_with("c_") {
        let file_name_chars: Vec<char> = removed_extension_file[13..].chars().collect();
        let date_str: String = file_name_chars.into_iter().collect();
        Ok(NaiveDate::parse_from_str(&date_str, FILENAME_DATE_FORMAT)?)
    } else {
        Err("the file name should starts with 'c_'".into())
    }
}

#[cfg(test)]
mod test_time_helper {
    use chrono::{Datelike, Timelike};

    use super::*;

    #[test]
    pub fn get_utc_time() {
        let _ = get_now_utc();
        assert!(true);
    }

    #[test]
    pub fn formatted_time_check() {
        let now = get_now_utc();
        let time_str = get_bson_time_str(now);
        assert!(time_str.contains("Z"));
        assert!(time_str.contains("-"));
    }

    #[test]
    pub fn test_convert_string_to_time() {
        let date = match convert_string_to_local("06/04/2023 06:00:00") {
            Ok(value) => value,
            Err(err) => panic!("{:?}", err),
        };
        assert_eq!(date.naive_local().day(), 4);
        assert_eq!(date.date_naive().month(), 6);
        assert_eq!(date.time().hour(), 6);

        let date2 = convert_string_to_local("12/23/2023 18:00:00").unwrap();
        assert_eq!(date2.naive_local().day(), 23);
        assert_eq!(date2.date_naive().month(), 12);
        assert_eq!(date2.time().hour(), 18);

        match convert_string_to_local("20/23/2023 18:00:00") {
            Ok(_) => panic!("shoudl not set to this message"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_extract_date_from_string() {
        let valid_string = "c_0123456789_23_03_31.xml";

        let valid_date = extract_date_from_xml_file(&valid_string).unwrap();
        assert_eq!(valid_date.year(), 2023);
        assert_eq!(valid_date.month(), 03);
        assert_eq!(valid_date.day(), 31);

        let invalid_string_without_start = "0123456789_23_03_31.xml";
        match extract_date_from_xml_file(&invalid_string_without_start) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }

        let invalid_string_time = "c_0123456789_23_31_03.xml";
        match extract_date_from_xml_file(&invalid_string_time) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}
