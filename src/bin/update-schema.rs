use std::{fs::File, io::Write};

use anyhow::Context;
use heck::ToSnakeCase;
use strum::IntoEnumIterator;
use windows_timezones::WindowsTimezone;

fn main() -> anyhow::Result<()> {
    let mut f = File::create("schema.sql").context("failed to create schema.sql")?;
    f.write_all(b"CREATE TYPE windows_timezone AS ENUM (\n")
        .context("failed to write header")?;

    let mut iter = WindowsTimezone::iter().peekable();
    while let Some(tz) = iter.next() {
        let stringified = format!("{tz:?}").to_snake_case();
        f.write_fmt(format_args!("    '{stringified}'"))
            .context("failed to write enum variant")?;
        if iter.peek().is_some() {
            f.write_all(b",\n")
                .context("failed to write line trailer")?;
        } else {
            f.write_all(b"\n").context("failed to write line trailer")?;
        }
    }

    f.write_all(b");\n").context("failed to write footer")?;

    Ok(())
}
