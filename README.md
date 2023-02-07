# Windows Timezones

This crate takes the list of Windows' (the OS) default timezones[^1] from the
[Unicode CLDR] project's [supplemental data files] and converts it into a Rust
enum that allows for retrieving the Windows timezone description and the
corresponding default tzdb ID.

The enum variants are guaranteed to be stay consistent within the same major
version of the crate.


## SQLx support

When the `sqlx` feature is enable `sqlx::Type` is derived for the
`WindowsTimezone` type.  The supported PostgreSQL type is kept in the
`schema.sql` file of this repository.  On major version updates you need to
ensure that your PostgreSQL type matches the `scheam.sql` file of the new
version!

## Features

- `chrono-tz`: Implements `From<WindowsTimezone> for chrono_tz::Tz`.
- `schemars`: Derives `schemars::JsonSchema`.
- `serde`: Derives `serde::Serialize` and `serde::Deserialize`.
- `sqlx`: Derives `sqlx::Type`.
- `strum`: Derives `strum::EnumIter`.

[Unicode CLDR]: https://github.com/unicode-org/cldr
[supplemental data files]: https://github.com/unicode-org/cldr/blob/main/common/supplemental/windowsZones.xml

[^1]: See the official [Windows documentation](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/default-time-zones).  Note that the CLDR list is more up-to-date than the actual Windows documentation page.
