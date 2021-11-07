use chrono::{DateTime, FixedOffset};
use std::error::Error;

pub type Timestamp = u64;

pub fn to_timestamp(s: &str) -> Result<Timestamp, Box<dyn Error>> {
    let dt = DateTime::parse_from_rfc2822(s)?;
    Ok(dt.timestamp() as Timestamp)
}

const SIMPLE_DATE: &str = "%y%m%d";

// parse a string of rfc2822 to simpledate
pub fn parse_utc8_simpledate(s: &str) -> Result<String, Box<dyn Error>> {
    let dt = DateTime::parse_from_rfc2822(s)?;
    let utc8: FixedOffset = FixedOffset::east(8 * 3600);
    let utc8_dt = dt.with_timezone(&utc8);
    Ok(utc8_dt.format(SIMPLE_DATE).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp() {
        let date1 = "Mon, 6 Sep 2021 08:00:02 +0800";
        let stmp = to_timestamp(date1).expect("timestamp failed");
        assert_eq!(1630886402, stmp);
    }

    #[test]
    fn date_utc8() {
        let date1 = "Mon, 6 Sep 2021 08:00:02 +0800";
        let res1 = parse_utc8_simpledate(date1);
        assert_eq!("210906", res1.unwrap());

        let date2 = "Mon, 6 Sep 2021 16:00:02 +0000";
        let res2 = parse_utc8_simpledate(date2);
        assert_eq!("210907", res2.unwrap());

        let date3 = "Mon, 6 Sep 2021 12:00:02 -0700";
        let res3 = parse_utc8_simpledate(date3);
        assert_eq!("210907", res3.unwrap());
    }
}
