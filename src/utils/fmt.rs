//! ## fmt
//!
//! format utilities

use std::time::SystemTime;

use jiff::Timestamp;

/// Format time using fmt string in utc time
///
/// Times outside the range `jiff` can represent saturate to the nearest representable bound.
pub fn fmt_time_utc(time: SystemTime, fmt: &str) -> String {
    let timestamp = Timestamp::try_from(time).unwrap_or(if time > SystemTime::UNIX_EPOCH {
        Timestamp::MAX
    } else {
        Timestamp::MIN
    });
    timestamp.strftime(fmt).to_string()
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_fmt_time() {
        let system_time: SystemTime = SystemTime::UNIX_EPOCH;
        assert_eq!(
            fmt_time_utc(system_time, "%Y-%m-%d %H:%M"),
            String::from("1970-01-01 00:00")
        );
    }

    #[test]
    fn should_saturate_out_of_range_time() {
        // Year ~14645, beyond the range jiff can represent.
        let far_future = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(400_000_000_000);
        assert_eq!(
            fmt_time_utc(far_future, "%Y%m%d%H%M.%S"),
            String::from("999912302200.00")
        );
    }
}
