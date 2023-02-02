# Windows Timezones

This crate takes the list of Windows timezones from the Unicode CLDR project's supplemental data files and converts it into a Rust enum that allows for retrieving the Windows timezone description and the corresponding default tzdb ID.

Optionally the timezone enum can be converted into [`chrono_tz::Tz`], or have its variants iterated over using the [`strum`] crate.

[`chrono_tz::Tz`]: https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html
[`strum`]: https://docs.rs/strum/latest/strum/trait.IntoEnumIterator.html
