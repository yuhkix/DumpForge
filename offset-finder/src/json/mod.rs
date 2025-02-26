#![cfg(feature = "json_input")]

use serde::{Deserialize, Serialize};

use crate::OffsetLocator;

#[derive(Serialize, Deserialize)]
pub struct OffsetLocatorJson {
    pub name: String,
    pub partial_match: Vec<String>,
    pub full_match: String,
    #[serde(default)]
    pub skip_offset_print: bool,
    #[serde(default)]
    pub allow_multiple_matches: bool,
}

impl<'a> Into<OffsetLocator<'a>> for &'a OffsetLocatorJson {
    fn into(self) -> OffsetLocator<'a> {
        let partials = self.partial_match.iter()
            .map(|pattern| pattern.as_str())
            .collect::<Vec<&str>>();

        OffsetLocator {
            name: &self.name,
            partial_match: partials,
            full_match: &self.full_match,
            skip_offset_print: self.skip_offset_print,
            allow_multiple_matches: self.allow_multiple_matches,
        }
    }
}