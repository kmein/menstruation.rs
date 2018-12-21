use super::{Group, MensaCode, Response};
use ansi_term::{Color, Style};
use regex::Regex;
use reqwest::{header, Client};
use scraper::{html::Html, ElementRef, Selector};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl TryFrom<Html> for Response<Mensa> {
    type Error = super::Error;
    fn try_from(html: Html) -> Result<Self, Self::Error> {
        let group_selector = Selector::parse("#itemsHochschulen .container-fluid").unwrap();
        let groups = html
            .select(&group_selector)
            .map(Group::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Response(groups))
    }
}

impl TryFrom<ElementRef<'_>> for Group<Mensa> {
    type Error = super::Error;
    fn try_from(html: ElementRef<'_>) -> Result<Self, Self::Error> {
        let group_name_selector = Selector::parse("h4").unwrap();
        let mensa_selector = Selector::parse(".row.row-top-percent-1.ptr[onclick]").unwrap();
        let name = html
            .select(&group_name_selector)
            .next()?
            .inner_html()
            .trim()
            .to_string();
        let mensas = html
            .select(&mensa_selector)
            .map(Mensa::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Group {
            name,
            items: mensas,
        })
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
            let onclick = html.value().attr("onclick")?;
            &in_call.captures(&onclick)?[1]
        })?;
        let address_html = html.select(&address_selector).next()?;
        let name = address_html.select(&name_selector).next()?.inner_html();
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

pub fn get() -> Result<Response<Mensa>, String> {
    match Client::new()
        .get("https://www.stw.berlin/mensen.html")
        .header(header::USER_AGENT, "Mozilla/5.0")
        .send()
    {
        Ok(mut response) => {
            assert!(response.status().is_success());
            match Response::try_from(Html::parse_document(&response.text().unwrap())) {
                Ok(codes) => Ok(codes),
                Err(e) => Err(format!("{:?}", e)),
            }
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}
