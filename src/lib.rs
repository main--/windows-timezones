#![doc = include_str!("../README.md")]

mod generated;

pub use generated::WindowsTimezone;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tz = WindowsTimezone::WEuropeStandardTime;

        let description = tz.description();
        assert!(description.starts_with("(UTC+01:00)"));
        assert!(description.contains("Berlin"));
        assert!(description.contains("Stockholm"));

        assert_eq!(tz.tzdb_id(), "Europe/Berlin");
    }

    #[cfg(feature = "chrono-tz")]
    #[test]
    fn chrono_tz_conversion() {
        assert_eq!(
            Into::<chrono_tz::Tz>::into(WindowsTimezone::WEuropeStandardTime),
            chrono_tz::Tz::Europe__Berlin
        );
    }

    #[cfg(feature = "strum")]
    #[test]
    fn strum() {
        use strum::IntoEnumIterator;

        let tzs = WindowsTimezone::iter().collect::<Vec<_>>();
        assert!(tzs.contains(&WindowsTimezone::WEuropeStandardTime));
    }
}
