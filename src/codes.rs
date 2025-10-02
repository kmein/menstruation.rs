use super::{error::Error, Group, MensaCode, Response};
use ansi_term::{Color, Style};
use regex::Regex;
use scraper::{html::Html, ElementRef, Selector};
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const CODES_DATA: &str = include_str!("../data/codes.json");

impl TryFrom<Html> for Response<Mensa> {
    type Error = Error;
    fn try_from(html: Html) -> Result<Self, Self::Error> {
        let group_selector = Selector::parse("#itemsHochschulen .container-fluid").unwrap();
        html.select(&group_selector)
            .map(Group::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map(Response)
    }
}

impl TryFrom<ElementRef<'_>> for Group<Mensa> {
    type Error = Error;
    fn try_from(html: ElementRef<'_>) -> Result<Self, Self::Error> {
        let group_name_selector = Selector::parse("h4").unwrap();
        let mensa_selector = Selector::parse(".row.row-top-percent-1.ptr[onclick]").unwrap();
        let name = html
            .select(&group_name_selector)
            .next()
            .ok_or(Error::Parse("Group<Mensa>::name".to_string()))?
            .inner_html()
            .trim()
            .to_string();
        html.select(&mensa_selector)
            .map(Mensa::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map(|items| Group { name, items })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Mensa {
    code: MensaCode,
    pub name: String,
    address: String,
}

impl Display for Mensa {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "{} {}",
            Color::Green.paint(format!("{:>4}", &self.code.to_string())),
            Style::new().italic().paint(&self.name),
        )
    }
}

impl TryFrom<ElementRef<'_>> for Mensa {
    type Error = Error;
    fn try_from(html: ElementRef<'_>) -> Result<Self, Self::Error> {
        let address_selector = Selector::parse(".addrcard").unwrap();
        let name_selector = Selector::parse("a.dummy div").unwrap();
        let code = MensaCode::from_str({
            let in_call = Regex::new(r"xhrLoad\('(\d+)'\)").unwrap();
            let onclick = html
                .value()
                .attr("onclick")
                .ok_or(Error::Parse("Mensa::code".to_string()))?;
            &in_call
                .captures(&onclick)
                .ok_or(Error::Parse("Mensa::code".to_string()))?[1]
        })
        .map_err(|e| Error::Parse(format!("Mensa::code\n< {}", e)))?;
        let address_html = html
            .select(&address_selector)
            .next()
            .ok_or(Error::Parse("Mensa::address".to_string()))?;
        let name = address_html
            .select(&name_selector)
            .next()
            .ok_or(Error::Parse("Mensa::name".to_string()))?
            .inner_html();
        let address = address_html
            .text()
            .skip(2)
            .map(|t| t.trim())
            .collect::<Vec<_>>()
            .join(", ");
        Ok(Mensa {
            code,
            name,
            address,
        })
    }
}

/// Filters the given `Response` by the given predicate, removing empty groups.
///
/// # Arguments
/// * `predicate` - A function that takes a reference to an item and returns `true` if the item
/// should be kept.
/// * `response` - The `Response` to filter.
///
/// # Returns
/// * A new `Response` containing only the items that match the predicate, with empty groups
/// removed.
pub fn filter_response<A>(predicate: impl Fn(&A) -> bool, response: Response<A>) -> Response<A> {
    Response(
        response
            .0
            .into_iter()
            .map(|group| {
                let items = group
                    .items
                    .into_iter()
                    .filter(&predicate)
                    .collect::<Vec<_>>();
                Group {
                    name: group.name,
                    items,
                }
            })
            .filter(|group| !group.items.is_empty())
            .collect(),
    )
}

pub fn get(pattern: Option<String>) -> Result<Response<Mensa>, Error> {
    let codes = serde_json::from_str::<Response<Mensa>>(CODES_DATA)
        .map_err(|e| Error::Parse(format!("Allergens\n< {}", e)))?;

    if let Some(p) = &pattern {
        return Ok(filter_response(
            |mensa| {
                let name_matches = mensa
                    .name
                    .to_lowercase()
                    .contains(&p.to_lowercase());
                let address_matches = mensa
                    .address
                    .to_lowercase()
                    .contains(&p.to_lowercase());
                name_matches || address_matches
            },
            codes,
        ));
    } else {
        return Ok(codes);
    }
}
