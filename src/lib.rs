#![feature(try_from, try_trait)]
extern crate ansi_term;
extern crate regex;
extern crate scraper;
extern crate serde;

#[macro_use]
extern crate serde_derive;

pub mod codes;
pub mod menu;
mod utility;

use ansi_term::{Colour, Style};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct MensaCode(pub u16);

impl From<u16> for MensaCode {
    fn from(code: u16) -> Self {
        MensaCode(code)
    }
}

impl Display for MensaCode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for MensaCode {
    type Err = std::num::ParseIntError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.parse::<u16>() {
            Ok(number) => Ok(MensaCode::from(number)),
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<Item>(Vec<NamedGroup<Item>>);

impl<Item> Display for Response<Item>
where
    Item: Display,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for group in &self.0 {
            write!(f, "{}", group)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct NamedGroup<Item> {
    name: String,
    items: Vec<Item>,
}

impl<Item> Display for NamedGroup<Item>
where
    Item: Display,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            Style::new().bold().paint(&self.name.to_uppercase(),)
        )?;
        for meal in &self.items {
            write!(f, "{}", meal)?;
        }
        writeln!(f)
    }
}
