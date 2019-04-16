#![feature(try_trait)]

pub mod allergens;
pub mod codes;
mod error;
pub mod menu;
mod utility;

use ansi_term::Style;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        string.parse::<u16>().map(MensaCode)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<Item>(Vec<Group<Item>>);

impl<Item> Response<Item> {
    pub fn filter(self, predicate: impl Fn(&Item) -> bool) -> Self {
        let mut groups = Vec::new();
        for group in self.0 {
            let meals = group
                .items
                .into_iter()
                .filter(&predicate)
                .collect::<Vec<_>>();
            if !meals.is_empty() {
                groups.push(Group {
                    items: meals,
                    ..group
                });
            }
        }
        Response(groups)
    }
}

impl<Item: Display> Display for Response<Item> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Ok(for group in &self.0 {
            write!(f, "{}", group)?;
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group<Item> {
    name: String,
    items: Vec<Item>,
}

impl<Item: Display> Display for Group<Item> {
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
