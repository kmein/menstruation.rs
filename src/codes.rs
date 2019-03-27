use super::{Group, MensaCode, Response};
use ansi_term::{Color, Style};
use regex::Regex;
use reqwest::{header, Client};
use scraper::{html::Html, ElementRef, Selector};
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl TryFrom<Html> for Response<Mensa> {
    type Error = super::Error;
    fn try_from(html: Html) -> Result<Self, Self::Error> {
        let group_selector = Selector::parse("#itemsHochschulen .container-fluid").unwrap();
        html.select(&group_selector)
            .map(Group::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map(Response)
    }
}

impl TryFrom<ElementRef<'_>> for Group<Mensa> {
    type Error = super::Error;
    fn try_from(html: ElementRef<'_>) -> Result<Self, Self::Error> {
        let group_name_selector = Selector::parse("h4").unwrap();
        let mensa_selector = Selector::parse(".row.row-top-percent-1.ptr[onclick]").unwrap();
        let name = html
            .select(&group_name_selector)
            .next()
            .ok_or(super::Error::Parse("Group<Mensa>::name".to_string()))?
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
    type Error = super::Error;
    fn try_from(html: ElementRef<'_>) -> Result<Self, Self::Error> {
        let address_selector = Selector::parse(".addrcard").unwrap();
        let name_selector = Selector::parse("a.dummy div").unwrap();
        let code = MensaCode::from_str({
            let in_call = Regex::new(r"xhrLoad\('(\d+)'\)").unwrap();
            let onclick = html
                .value()
                .attr("onclick")
                .ok_or(super::Error::Parse("Mensa::code".to_string()))?;
            &in_call
                .captures(&onclick)
                .ok_or(super::Error::Parse("Mensa::code".to_string()))?[1]
        })
        .map_err(|e| super::Error::Parse(format!("Mensa::code\n< {}", e)))?;
        let address_html = html
            .select(&address_selector)
            .next()
            .ok_or(super::Error::Parse("Mensa::address".to_string()))?;
        let name = address_html
            .select(&name_selector)
            .next()
            .ok_or(super::Error::Parse("Mensa::name".to_string()))?
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

pub fn get() -> Result<Response<Mensa>, super::Error> {
    match Client::new()
        .get("https://www.stw.berlin/mensen.html")
        .header(header::USER_AGENT, "Mozilla/5.0")
        .send()
    {
        Ok(mut response) => {
            assert!(response.status().is_success());
            let content = response.text().map_err(|e| {
                super::Error::Net(format!("network response text not found\n< {}", e))
            })?;
            Response::try_from(Html::parse_document(&content))
                .map_err(|e| super::Error::Parse(format!("Response<Mensa>\n< {}", e)))
        }
        Err(e) => Err(super::Error::Net(e.to_string())),
    }
}
