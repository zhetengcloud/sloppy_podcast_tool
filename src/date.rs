use chrono::DateTime;
use std::error::Error;

pub type Timestamp = u64;

pub fn to_timestamp(s: &str) -> Result<Timestamp, Box<dyn Error>> {
    let dt = DateTime::parse_from_rfc2822(s)?;
    Ok(dt.timestamp() as Timestamp)
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
}
