use anyhow::Context;
use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Span};
use quick_xml::events::Event;
use quote::quote;

const CLDR_VERSION: &'static str = "42";

fn main() -> anyhow::Result<()> {
    let url = format!("https://raw.githubusercontent.com/unicode-org/cldr/release-{CLDR_VERSION}/common/supplemental/windowsZones.xml");
    let body = reqwest::blocking::get(url)
        .context("failed to get CLDR data file")?
        .text()
        .context("failed to get CLDR data file text content")?;

    let mut reader = quick_xml::reader::Reader::from_str(&body);
    reader.check_comments(true);

    let mut buf = Vec::new();
    let mut state = State::default();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("error at position {}: {:?}", reader.buffer_position(), e),
            Ok(event) => {
                if !state.parse_event(event).unwrap() {
                    break;
                }
            }
        };
    }

    println!("{}", state.generate_enum());

    Ok(())
}

// Convert all '/' to '__', all '+' to 'Plus' and '-' to 'Minus', unless
// it's a hyphen, in which case remove it. This is so the names can be used
// as rust identifiers.
//
// Copied from chrono-tz.
// https://github.com/chronotope/chrono-tz/blob/3e4adb2891e1cb9167a32c531cdb2f37eb0aaa30/chrono-tz-build/src/lib.rs#L50-L64
fn convert_bad_chars(name: &str) -> String {
    let name = name.replace("/", "__").replace("+", "Plus");
    if let Some(pos) = name.find('-') {
        if name[pos + 1..]
            .chars()
            .next()
            .map(char::is_numeric)
            .unwrap_or(false)
        {
            name.replace("-", "Minus")
        } else {
            name.replace("-", "")
        }
    } else {
        name
    }
}

#[derive(Debug)]
struct Timezone {
    user_friendly_name: String,
    windows_name: String,
    tzdb_id: String,
}

#[derive(Default, Debug)]
struct State {
    timezones: Vec<Timezone>,
    current_friendly_name: Option<String>,
    current_timezone: Option<Timezone>,
}

impl State {
    fn parse_event(&mut self, event: Event) -> anyhow::Result<bool> {
        match event {
            Event::Eof => {
                if let Some(current) = self.current_timezone.take() {
                    self.timezones.push(current);
                }
                self.current_friendly_name.take();
                return Ok(false);
            }
            Event::Comment(comment) => {
                if let Some(current) = self.current_timezone.take() {
                    self.timezones.push(current);
                }
                let timezone = String::from_utf8_lossy(&comment.into_inner())
                    .to_string()
                    .trim()
                    .to_string();
                self.current_friendly_name = Some(timezone);
            }
            Event::Empty(tag) if tag.local_name().as_ref() == b"mapZone" => {
                let windows_name = String::from_utf8_lossy(
                    &tag.try_get_attribute("other")?
                        .context("missing 'other' attribute")?
                        .value,
                )
                .to_string();

                let territory = String::from_utf8_lossy(
                    &tag.try_get_attribute("territory")?
                        .context("missing 'territory' attribute")?
                        .value,
                )
                .to_string();
                let tzdb_id = String::from_utf8_lossy(
                    &tag.try_get_attribute("type")?
                        .context("missing 'other' attribute")?
                        .value,
                )
                .to_string();

                if territory == "001" {
                    let user_friendly_name = self
                        .current_friendly_name
                        .as_deref()
                        .context("missing time zone header")?
                        .to_string();
                    self.current_timezone = Some(Timezone {
                        user_friendly_name,
                        windows_name,
                        tzdb_id,
                    });
                }
            }
            _ => {}
        }

        Ok(true)
    }

    fn generate_enum(self) -> String {
        let mut type_variants = Vec::new();
        let mut defaults = Vec::new();
        let mut chrono_tz_variants = Vec::new();
        let mut timezone_descriptions = Vec::new();
        let mut tzdb_ids = Vec::new();
        for timezone in self.timezones {
            type_variants.push(Ident::new(
                &convert_bad_chars(&timezone.windows_name).to_upper_camel_case(),
                Span::call_site(),
            ));
            if timezone.windows_name == "UTC" {
                defaults.push(quote! { #[default] });
            } else {
                defaults.push(quote! {});
            }
            chrono_tz_variants.push(Ident::new(
                &convert_bad_chars(&timezone.tzdb_id),
                Span::call_site(),
            ));
            timezone_descriptions.push(timezone.user_friendly_name);
            tzdb_ids.push(timezone.tzdb_id);
        }

        let quoted = quote! {
            #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
            #[cfg_attr(feature = "strum", derive(::strum::EnumIter))]
            pub enum Timezone {
                #(
                    #[doc = #timezone_descriptions]
                    #defaults
                    #type_variants
                ),*
            }

            impl Timezone {
                pub fn description(self) -> &'static str {
                    match self {
                        #(
                            Self::#type_variants => #timezone_descriptions
                        ),*
                    }
                }

                pub fn tzdb_id(self) -> &'static str {
                    match self {
                        #(
                            Self::#type_variants => #tzdb_ids
                        ),*
                    }
                }
            }

            #[cfg(feature = "chrono-tz")]
            impl From<Timezone> for ::chrono_tz::Tz {
                fn from(value: Timezone) -> Self {
                    match value {
                        #(
                            Timezone::#type_variants => ::chrono_tz::Tz::#chrono_tz_variants
                        ),*
                    }
                }
            }
        };

        quoted.to_string()
    }
}
