
use chrono::{DateTime, Utc, format::{DelayedFormat, StrftimeItems}, NaiveDate, NaiveTime,NaiveDateTime, Local};

const BSON_TIME_FORMAT: &str="%Y-%m-%dT%H:%M:%SZ";

pub fn get_now_utc() -> DateTime<Utc> {
    Utc::now()
}

pub fn get_bson_time<'a>(dt: DateTime<Utc>) -> DelayedFormat<StrftimeItems<'a>>  {
    dt.format(BSON_TIME_FORMAT)
}

pub fn get_bson_time_str<'a>(dt: DateTime<Utc>) -> String { 
    get_bson_time(dt).to_string()
}

fn create_datetime(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> NaiveDateTime {
    let naive_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let naive_time = NaiveTime::from_hms_opt(hour, minute, second).unwrap();

    NaiveDateTime::new(naive_date, naive_time)
}

pub fn create_datetime_utc(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> DateTime<Utc> {
    DateTime::from_naive_utc_and_offset(
        create_datetime(year, month, day, hour, minute, second), 
        Utc
    )
}

pub fn convert_local_to_utc(datetime: DateTime<Local>) -> DateTime<Utc> {
    datetime.with_timezone(&Utc)
}

#[cfg(test)]
mod test_time_helper {
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
}