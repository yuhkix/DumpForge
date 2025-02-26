use std::collections::HashMap;

use goblin::pe::section_table::SectionTable;
use log::{debug, warn};
use patternscanner::PatternScannerBuilder;

#[cfg(feature = "json_input")]
pub mod json;

pub struct OffsetLocator<'a> {
    pub name: &'a str,
    pub partial_match: Vec<&'a str>,
    pub full_match: &'a str,
    pub skip_offset_print: bool,
    pub allow_multiple_matches: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Offset for: {0} not found")]
    NotFound(String),
    #[error("Too many matches found for: {0}")]
    TooManyMatches(String),
    #[error("PE Utils: {0}")]
    PeUtils(#[from] pe_utils::Error),
    #[error("Toml Error: {0}")]
    PatternScanner(#[from] patternscanner::PatternScannerError),
}

#[inline]
fn find_pattern(image_base: usize,
                sections: &[SectionTable],
                data: &[u8],
                pattern: &str,
                name: &str,
                allow_multiple_matches: bool) -> Result<(usize, usize), Error> {
    let items = find_all_pattern(image_base, sections, data, pattern, name)?;
    if items.len() == 1 {
        return Ok((items[0].0, items[0].1));
    }

    if allow_multiple_matches {
        return Ok(items[0]);
    }
    for item in items {
        warn!("Possible candidate for: {}, 0x{:02X?}", name, item.1);
    }
    Err(Error::TooManyMatches(name.to_string()))
}

#[inline]
fn find_all_pattern(image_base: usize,
                    sections: &[SectionTable],
                    data: &[u8],
                    pattern: &str,
                    name: &str) -> Result<Vec<(usize, usize)>, Error> {
    let items = PatternScannerBuilder::builder()
        .with_bytes(data)
        .build()
        .scan_all(pattern)?;

    if items.is_empty() {
        return Err(Error::NotFound(name.to_string()));
    }

    let mut output: Vec<(usize, usize)> = Vec::with_capacity(items.len());
    for item in items {
        output.push((item, pe_utils::resolve_symbol(image_base, sections, item)?));
    }
    Ok(output)
}

#[inline]
fn find_patterns(image_base: usize,
                 sections: &[SectionTable],
                 data: &[u8],
                 patterns: &[&str],
                 name: &str,
                 allow_multiple_matches: bool) -> Result<(usize, usize), Error> {
    for pattern in patterns {
        let result = match find_pattern(image_base, sections, data, pattern, name, allow_multiple_matches) {
            Ok(result) => Ok(result),
            Err(Error::NotFound(_)) => continue,
            Err(err) => Err(err)
        }?;
        debug!("Partial pattern match with: {}", pattern);
        return Ok(result);
    }
    Err(Error::NotFound(name.to_string()))
}

#[inline]
fn find_all_patterns(image_base: usize,
                     sections: &[SectionTable],
                     data: &[u8],
                     patterns: &[&str],
                     name: &str) -> Result<HashMap<usize, Vec<(usize, usize)>>, Error> {
    let mut output = HashMap::new();
    for i in 0..patterns.len() {
        let result = match find_all_pattern(image_base, sections, data, &patterns[i], name) {
            Ok(result) => Ok(result),
            Err(Error::NotFound(_)) => continue,
            Err(err) => Err(err)
        }?;
        debug!("Partial pattern match with: {}", &patterns[i]);
        output.insert(i, result);
    }
    match output.is_empty() {
        true => Err(Error::NotFound(name.to_string())),
        false => Ok(output)
    }
}

impl<'a> OffsetLocator<'a> {
    pub fn find_offset(&self,
                       image_base: usize,
                       sections: &[SectionTable],
                       executable: &[u8]) -> Result<(usize, usize, bool), Error> {
        match find_pattern(
            image_base,
            sections,
            executable,
            self.full_match,
            self.name,
            self.allow_multiple_matches,
        ) {
            Ok(result) => Ok((result.0, result.1, true)),
            Err(Error::NotFound(_)) => {
                let result = find_patterns(
                    image_base,
                    sections,
                    executable,
                    &self.partial_match,
                    self.name,
                    self.allow_multiple_matches,
                )?;
                Ok((result.0, result.1, false))
            }
            Err(err) => Err(err)
        }
    }

    pub fn find_all_partial_only(&self,
                                 image_base: usize,
                                 sections: &[SectionTable],
                                 executable: &[u8]) -> Result<HashMap<usize, Vec<(usize, usize)>>, Error> {
        find_all_patterns(
            image_base,
            sections,
            executable,
            &self.partial_match,
            self.name,
        )
    }
}