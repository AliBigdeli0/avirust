
use chrono::{DateTime, Utc, format::{DelayedFormat, StrftimeItems}};

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