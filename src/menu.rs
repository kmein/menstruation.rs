use super::utility;
use super::{Group, MensaCode, Response};
use ansi_term::{Colour, Style};
use chrono::{Local, NaiveDate, ParseError};
use regex::Regex;
use reqwest::{header, Client};
use rocket::request::{FromQuery, Query};
use scraper::{html::Html, ElementRef, Selector};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct Cents(u64);

impl Cents {
    fn from_euro(euro: f64) -> Self {
        Cents((euro * 100f64) as u64)
    }
}

impl From<u64> for Cents {
    fn from(cents: u64) -> Self {
        Cents(cents)
    }
}

impl Display for Cents {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let Cents(total_cents) = self;
        let euros = total_cents / 100;
        let cents = total_cents % 100;
        write!(f, "{},{:02} €", euros, cents)
    }
}

impl FromStr for Cents {
    type Err = std::num::ParseFloatError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.parse() {
            Ok(float) => Ok(Cents::from_euro(float)),
            Err(err) => Err(err),
        }
    }
}

impl TryFrom<Html> for Response<Meal> {
    type Error = super::Error;
    fn try_from(html: Html) -> Result<Self, Self::Error> {
        let group_selector = Selector::parse(".splGroupWrapper").unwrap();
        let groups = html
            .select(&group_selector)
            .map(Group::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| super::Error::Parse(format!("Response<Meal>::0\n< {}", e)))?;
        Ok(Response(groups))
    }
}

impl TryFrom<ElementRef<'_>> for Group<Meal> {
    type Error = super::Error;
    fn try_from(html: ElementRef<'_>) -> Result<Self, Self::Error> {
        let group_name_selector = Selector::parse(".splGroup").unwrap();
        let meal_selector = Selector::parse(".splMeal").unwrap();

        let name = html
            .select(&group_name_selector)
            .next()
            .ok_or(super::Error::Parse("Group::name".to_string()))?
            .inner_html();
        let meals = html
            .select(&meal_selector)
            .map(Meal::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| super::Error::Parse(format!("Group::items\n< {}", e)))?;
        Ok(Group { name, items: meals })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meal {
    pub name: String,
    pub color: Color,
    pub tags: HashSet<Tag>,
    pub price: Option<Price>,
    pub allergens: HashSet<String>,
}

impl Display for Meal {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        fn to_ansi(color: &Color) -> Colour {
            match color {
                Color::Green => Colour::Green,
                Color::Red => Colour::Red,
                Color::Yellow => Colour::Yellow,
            }
        }
        writeln!(
            f,
            "[{}] {} {}",
            format!(
                "{}",
                match &self.price {
                    None => 0.into(),
                    Some(p) => p.student,
                }
            ),
            to_ansi(&self.color).paint(&self.name),
            self.tags
                .iter()
                .map(|tag| format!("{}", tag))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl TryFrom<ElementRef<'_>> for Meal {
    type Error = super::Error;
    fn try_from(html: ElementRef<'_>) -> Result<Self, Self::Error> {
        let icon_selector = Selector::parse("img[src].splIcon").unwrap();
        let meal_name_selector = Selector::parse("span.bold").unwrap();
        let allergen_selector = Selector::parse(".toolt").unwrap();

        let icons_html = html
            .select(&icon_selector)
            .map(|img| img.value().attr("src"))
            .collect::<Option<Vec<_>>>()
            .ok_or(super::Error::Parse("Meal icons".to_string()))?;
        let (color_htmls, tag_htmls) =
            utility::partition(|&src| src.contains("ampel"), &icons_html);
        let color =
            parse_color(color_htmls[0]).ok_or(super::Error::Parse("Meal::color".to_string()))?;
        let tags = tag_htmls
            .iter()
            .map(|&src| parse_tag(src))
            .collect::<Option<HashSet<_>>>()
            .ok_or(super::Error::Parse("Meal::tags".to_string()))?;
        let meal_name = html
            .select(&meal_name_selector)
            .next()
            .ok_or(super::Error::Parse("Meal::name".to_string()))?
            .inner_html()
            .trim()
            .to_string();
        let price = Price::try_from(html).ok();
        let allergens = {
            let parenthesized = Regex::new(r"\((.*)\)").unwrap();
            let allergens_html = html
                .select(&allergen_selector)
                .next()
                .expect("No allergens found")
                .inner_html();
            if let Some(captures) = parenthesized.captures(&allergens_html) {
                String::from(&captures[1])
                    .split(", ")
                    .map(String::from)
                    .collect()
            } else {
                HashSet::new()
            }
        };
        Ok(Meal {
            name: meal_name,
            tags,
            color,
            price,
            allergens,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    Red,
}

impl FromStr for Color {
    type Err = String;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "red" => Ok(Color::Red),
            _ => Err(format!(
                "Falsche Farbe: {}. Bitte nutze green, yellow oder red.",
                string
            )),
        }
    }
}

fn parse_color(uri: &str) -> Option<Color> {
    match uri {
        "/vendor/infomax/mensen/icons/ampel_gelb_70x65.png" => Some(Color::Yellow),
        "/vendor/infomax/mensen/icons/ampel_gruen_70x65.png" => Some(Color::Green),
        "/vendor/infomax/mensen/icons/ampel_rot_70x65.png" => Some(Color::Red),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tag {
    #[serde(rename = "vegetarian")]
    Vegetarian,
    #[serde(rename = "vegan")]
    Vegan,
    #[serde(rename = "organic")]
    Organic,
    #[serde(rename = "sustainable fishing")]
    SustainableFishing,
    #[serde(rename = "climate friendly")]
    ClimateFriendly,
}

impl FromStr for Tag {
    type Err = String;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "vegan" => Ok(Tag::Vegan),
            "vegetarian" => Ok(Tag::Vegetarian),
            "organic" => Ok(Tag::Organic),
            "climate friendly" => Ok(Tag::ClimateFriendly),
            "sustainable fishing" => Ok(Tag::SustainableFishing),
            _ => Err(format!(
                "Wrong tag: {}. Please use vegan, vegetarian, organic, climate friendly or sustainable fishing.",
                string
            )),
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Style::new().italic().paint(match self {
                Tag::Vegetarian => "vegetarian",
                Tag::Vegan => "vegan",
                Tag::Organic => "organic",
                Tag::SustainableFishing => "sustainable fishing",
                Tag::ClimateFriendly => "climate friendly",
            })
        )
    }
}

fn parse_tag(uri: &str) -> Option<Tag> {
    match uri {
        "/vendor/infomax/mensen/icons/1.png" => Some(Tag::Vegetarian),
        "/vendor/infomax/mensen/icons/15.png" => Some(Tag::Vegan),
        "/vendor/infomax/mensen/icons/18.png" => Some(Tag::Organic),
        "/vendor/infomax/mensen/icons/38.png" => Some(Tag::SustainableFishing),
        "/vendor/infomax/mensen/icons/43.png" => Some(Tag::ClimateFriendly),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Price {
    pub student: Cents,
    employee: Cents,
    guest: Cents,
}

impl TryFrom<ElementRef<'_>> for Price {
    type Error = super::Error;
    fn try_from(html: ElementRef<'_>) -> Result<Self, Self::Error> {
        let price_selector = Selector::parse("div.text-right").unwrap();
        let price_raw = html
            .select(&price_selector)
            .next()
            .ok_or(super::Error::Parse("Meal::price".to_string()))?;
        let prices: Vec<_> = price_raw
            .inner_html()
            .replace("€", "")
            .replace(",", ".")
            .trim()
            .split('/')
            .map(|p| p.parse::<f64>().map(Cents::from_euro))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| super::Error::Parse(format!("Meal::price\n< {}", e)))?;
        Ok(Price {
            student: prices[0],
            employee: prices[1],
            guest: prices[2],
        })
    }
}

pub fn get(options: &MenuOptions) -> Result<Response<Meal>, super::Error> {
    match Client::new()
        .post("https://www.stw.berlin/xhr/speiseplan-wochentag.html")
        .form(&[
            ("week", "now"),
            (
                "date",
                &options
                    .date
                    .unwrap_or_else(|| Local::today().naive_local())
                    .format("%Y-%m-%d")
                    .to_string(),
            ),
            ("resources_id", &options.mensa.0.to_string()),
        ])
        .header(header::USER_AGENT, "Mozilla/5.0")
        .send()
    {
        Ok(mut response) => {
            assert!(response.status().is_success());
            Response::try_from(Html::parse_fragment(&response.text().unwrap()))
                .map_err(|e| super::Error::Parse(format!("Response<Meal>\n< {}", e)))
                .map(|response| query(options, response))
        }
        Err(e) => Err(super::Error::Net(e.to_string())),
    }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct MenuOptions {
    #[structopt(short, long, parse(try_from_str))]
    /// Displays only meals with the specified colors
    pub colors: Vec<Color>,
    #[structopt(short, long, parse(try_from_str))]
    /// Displays only meals with the specified tags
    pub tags: Vec<Tag>,
    #[structopt(short = "p", long)]
    /// Displays no meals more expensive than a given price
    pub max_price: Option<Cents>,
    #[structopt(short, long)]
    /// Displays no meals containing the specified allergens
    pub allergens: Vec<String>,
    #[structopt(short, long, parse(try_from_str = "parse_iso_date"))]
    /// Chooses the menu date
    pub date: Option<NaiveDate>,
    #[structopt(short, long, default_value = "191")]
    /// Chooses a dining facility
    pub mensa: MensaCode,
}

impl<'a> FromQuery<'a> for MenuOptions {
    type Error = ();

    fn from_query(query: Query<'a>) -> Result<Self, Self::Error> {
        fn query_values<T: FromStr>(key: &str, query: &Query) -> Vec<T> {
            query
                .clone()
                .filter_map(|item| {
                    if item.key == key {
                        FromStr::from_str(item.value).ok()
                    } else {
                        None
                    }
                })
                .collect()
        }

        if let Some(Ok(mensa)) = query
            .clone()
            .find(|item| item.key == "mensa")
            .map(|item| item.value.parse().map(|code: u16| code.into()))
        {
            Ok(MenuOptions {
                colors: query_values("color", &query),
                tags: query_values("tag", &query),
                max_price: query
                    .clone()
                    .find(|item| item.key == "max_price")
                    .map(|item| item.value.parse().map(|code: u64| code.into()))
                    .and_then(|x| x.ok()),
                allergens: query_values("allergen", &query),
                date: query
                    .clone()
                    .find(|item| item.key == "date")
                    .map(|item| parse_iso_date(item.value))
                    .and_then(|x| x.ok()),
                mensa,
            })
        } else {
            Err(())
        }
    }
}

fn parse_iso_date(string: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(string, "%Y-%m-%d")
}

fn query(options: &MenuOptions, menu: Response<Meal>) -> Response<Meal> {
    super::filter_response(menu, |meal| {
        let price_ok = if let Some(max) = &options.max_price {
            if let Some(price) = &meal.price {
                price.student <= *max
            } else {
                false
            }
        } else {
            true
        };
        let colors_ok = options.colors.is_empty() || options.colors.contains(&meal.color);
        let tags_ok = options.tags.is_empty()
            || meal.tags.iter().any(|tag| {
                options.tags.contains(tag)
                    || (tag == &Tag::Vegan && options.tags.contains(&Tag::Vegetarian))
            });
        let allergens_ok = meal
            .allergens
            .iter()
            .all(|allergen| !options.allergens.contains(allergen));
        price_ok && colors_ok && tags_ok && allergens_ok
    })
}
