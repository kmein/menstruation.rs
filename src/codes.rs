use super::{MensaCode, NamedGroup, Response};
use ansi_term::{Colour, Style};
use regex::Regex;
use reqwest::{header, Client};
use scraper::{html::Html, ElementRef, Selector};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl TryFrom<Html> for Response<Mensa> {
    type Error = std::option::NoneError;
    fn try_from(html: Html) -> Result<Self, Self::Error> {
        let group_selector = Selector::parse("#itemsHochschulen .container-fluid").unwrap();
        let groups = html
            .select(&group_selector)
            .map(NamedGroup::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Response(groups))
    }
}

impl TryFrom<ElementRef<'_>> for NamedGroup<Mensa> {
    type Error = std::option::NoneError;
    fn try_from(html: ElementRef<'_>) -> Result<Self, Self::Error> {
        let group_name_selector = Selector::parse(".h4").unwrap();
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
        Ok(NamedGroup {
            name,
            items: mensas,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Mensa {
    code: MensaCode,
    address: String,
}

impl Display for Mensa {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:4} {}",
            self.code,
            Style::new().italic().paint(&self.address)
        )
    }
}

impl TryFrom<ElementRef<'_>> for Mensa {
    type Error = std::option::NoneError;
    fn try_from(html: ElementRef<'_>) -> Result<Self, Self::Error> {
        let address_selector = Selector::parse("addrcard").unwrap();

        let code = MensaCode::from_str(html.value().attr("onclick")?).ok()?; // [9:12]
        let address = html
            .select(&address_selector)
            .next()?
            .inner_html()
            .trim()
            .replace('\n', " /")
            .replace(r"\s+", " ");
        Ok(Mensa { code, address })
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
