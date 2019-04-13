use super::{error::Error, Group};
use ansi_term::{Color, Style};
use regex::Regex;
use reqwest::{header, Client};
use scraper::{Html, Selector};
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
pub struct Allergen {
    name: String,
    number: u8,
    index: Option<char>,
}

impl TryFrom<Html> for Group<Allergen> {
    type Error = Error;
    fn try_from(html: Html) -> Result<Self, Self::Error> {
        let group_selector = Selector::parse("div.col-sm-6 > ul > li").unwrap();
        let number_name =
            Regex::new(r"(?P<number>\d+)(?P<index>\w?) - (?P<name>(\w|\s|[.()])+)").unwrap();

        Ok(Group {
            name: "allergens".into(),
            items: {
                let mut result = Vec::new();
                for element_ref in html.select(&group_selector) {
                    let text = element_ref.inner_html();
                    for captures in number_name.captures_iter(&text) {
                        result.push(Allergen {
                            number: captures["number"]
                                .parse::<u8>()
                                .map_err(|e| Error::Parse(format!("Allergen\n< {}", e)))?,
                            name: captures["name"].into(),
                            index: captures["index"].chars().nth(0),
                        })
                    }
                }
                result.sort_by_key(|a| a.number.clone());
                result
            },
        })
    }
}

impl Display for Allergen {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "{}{} {}",
            Color::Green.paint(format!("{:>2}", &self.number)),
            Color::Green.paint(format!("{}", &self.index.unwrap_or(' '))),
            Style::new().italic().paint(&self.name),
        )
    }
}

pub fn get() -> Result<Group<Allergen>, Error> {
    match Client::new()
        .get("https://www.stw.berlin/mensen.html")
        .header(header::USER_AGENT, "Mozilla/5.0")
        .send()
    {
        Ok(mut response) => {
            assert!(response.status().is_success());
            let content = response
                .text()
                .map_err(|e| Error::Net(format!("network response text not found\n< {}", e)))?;
            Group::try_from(Html::parse_document(&content))
                .map_err(|e| Error::Parse(format!("Group<Allergen>\n< {}", e)))
        }
        Err(e) => Err(Error::Net(e.to_string())),
    }
}
