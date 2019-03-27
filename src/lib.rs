#![feature(try_from, try_trait)]

pub mod codes;
pub mod menu;
mod utility;

use ansi_term::{Colour::Red, Style};
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    Parse(String),
    Net(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let error_style = Red.bold();
        match self {
            Error::Parse(message) => write!(f, "{} {}", error_style.paint("PARSE ERROR"), message),
            Error::Net(message) => write!(f, "{} {}", error_style.paint("NETWORK ERROR"), message),
        }
    }
}

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
        string.parse::<u16>().map(MensaCode)
    }
}

pub fn filter_response<Item>(
    menu: Response<Item>,
    predicate: impl Fn(&Item) -> bool,
) -> Response<Item> {
    let mut groups = Vec::new();
    for group in menu.0 {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<Item>(Vec<Group<Item>>);

impl<Item: Display> Display for Response<Item> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Ok(for group in &self.0 {
            write!(f, "{}", group)?;
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Group<Item> {
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
