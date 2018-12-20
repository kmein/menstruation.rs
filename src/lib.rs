#![feature(try_from)]
extern crate ansi_term;
extern crate regex;
extern crate scraper;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod utility;

use ansi_term::{Colour, Style};
use chrono::{Local, NaiveDate};
use regex::Regex;
use reqwest::{header, Client, Response};
use scraper::{html::Html, ElementRef, Selector};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct Cents(u64);

impl Cents {
    fn from_euro(euro: f32) -> Self {
        Cents((euro * 100f32) as u64)
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuResponse(pub Vec<MealGroup>);

impl Display for MenuResponse {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut result = Ok(());
        for group in &self.0 {
            result = write!(f, "{}", group);
        }
        result
    }
}

impl From<Html> for MenuResponse {
    fn from(html: Html) -> Self {
        let group_selector = Selector::parse(".splGroupWrapper").unwrap();
        let groups = html.select(&group_selector).map(MealGroup::from).collect();
        MenuResponse(groups)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MealGroup {
    pub name: String,
    pub meals: Vec<Meal>,
}

impl Display for MealGroup {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            Style::new().bold().paint(&self.name.to_uppercase(),)
        );
        for meal in &self.meals {
            write!(f, "{}", meal);
        }
        writeln!(f)
    }
}

impl From<ElementRef<'_>> for MealGroup {
    fn from(html: ElementRef<'_>) -> Self {
        let group_name_selector = Selector::parse(".splGroup").unwrap();
        let meal_selector = Selector::parse(".splMeal").unwrap();

        let group_name = html
            .select(&group_name_selector)
            .next()
            .expect("No group name found")
            .inner_html();
        let meals = html.select(&meal_selector).map(Meal::from).collect();
        MealGroup {
            name: group_name,
            meals,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meal {
    pub name: String,
    pub color: MealColor,
    pub tags: HashSet<MealTag>,
    pub price: Option<MealPrice>,
    pub allergens: HashSet<String>,
}

impl Display for Meal {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        fn to_ansi(color: &MealColor) -> Colour {
            match color {
                MealColor::Green => Colour::Green,
                MealColor::Red => Colour::Red,
                MealColor::Yellow => Colour::Yellow,
            }
        }
        writeln!(
            f,
            "[{}] {} {}",
            if let Some(price) = &self.price {
                format!("{}", price.student)
            } else {
                "      ".to_string()
            },
            to_ansi(&self.color).paint(&self.name),
            self.tags
                .iter()
                .map(|tag| format!("{}", tag))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl From<ElementRef<'_>> for Meal {
    fn from(html: ElementRef<'_>) -> Self {
        let icon_selector = Selector::parse("img.splIcon").unwrap();
        let meal_name_selector = Selector::parse("span.bold").unwrap();
        let allergen_selector = Selector::parse(".toolt").unwrap();

        let icons_html = html
            .select(&icon_selector)
            .map(|img| img.value().attr("src").expect("Icon has no src"));
        let (color_htmls, tag_htmls) = utility::partition(|&src| src.contains("ampel"), icons_html);
        let color = parse_color(color_htmls[0]).expect("Unknown color icon");
        let tags = tag_htmls
            .iter()
            .map(|&src| parse_tag(src))
            .collect::<Option<HashSet<_>>>()
            .expect("Unknown tag icon");
        let meal_name = html
            .select(&meal_name_selector)
            .next()
            .expect("No meal name found")
            .inner_html()
            .trim()
            .to_string();
        let price = MealPrice::try_from(html).ok();
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
        Meal {
            name: meal_name,
            tags,
            color,
            price,
            allergens,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MealColor {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    Red,
}

impl FromStr for MealColor {
    type Err = String;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "grün" => Ok(MealColor::Green),
            "gelb" => Ok(MealColor::Yellow),
            "rot" => Ok(MealColor::Red),
            _ => Err(format!(
                "Falsche Farbe: {}. Bitte nutze grün, gelb oder rot.",
                string
            )),
        }
    }
}

fn parse_color(uri: &str) -> Option<MealColor> {
    match uri {
        "/vendor/infomax/mensen/icons/ampel_gelb_70x65.png" => Some(MealColor::Yellow),
        "/vendor/infomax/mensen/icons/ampel_gruen_70x65.png" => Some(MealColor::Green),
        "/vendor/infomax/mensen/icons/ampel_rot_70x65.png" => Some(MealColor::Red),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MealTag {
    #[serde(rename = "vegetarian")]
    Vegetarian,
    #[serde(rename = "vegan")]
    Vegan,
    #[serde(rename = "organic")]
    Organic,
    #[serde(rename = "sustainable fishing")]
    SustainableFishing,
    #[serde(rename = "climate")]
    ClimateFriendly,
}

impl FromStr for MealTag {
    type Err = String;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "vegan" => Ok(MealTag::Vegan),
            "vegetarisch" => Ok(MealTag::Vegetarian),
            "bio" => Ok(MealTag::Organic),
            "öko" => Ok(MealTag::ClimateFriendly),
            "nachhaltig" => Ok(MealTag::SustainableFishing),
            _ => Err(format!(
                "Falsches Tag: {}. Bitte nutze vegan, vegetarisch, bio, öko oder nachhaltig.",
                string
            )),
        }
    }
}

impl Display for MealTag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Style::new().italic().paint(match self {
                MealTag::Vegetarian => "vegetarisch",
                MealTag::Vegan => "vegan",
                MealTag::Organic => "bio",
                MealTag::SustainableFishing => "nachhaltig",
                MealTag::ClimateFriendly => "öko",
            })
        )
    }
}

fn parse_tag(uri: &str) -> Option<MealTag> {
    match uri {
        "/vendor/infomax/mensen/icons/1.png" => Some(MealTag::Vegetarian),
        "/vendor/infomax/mensen/icons/15.png" => Some(MealTag::Vegan),
        "/vendor/infomax/mensen/icons/18.png" => Some(MealTag::Organic),
        "/vendor/infomax/mensen/icons/38.png" => Some(MealTag::SustainableFishing),
        "/vendor/infomax/mensen/icons/43.png" => Some(MealTag::ClimateFriendly),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MealPrice {
    pub student: Cents,
    employee: Cents,
    guest: Cents,
}

impl TryFrom<ElementRef<'_>> for MealPrice {
    type Error = ();
    fn try_from(html: ElementRef<'_>) -> Result<Self, ()> {
        let price_selector = Selector::parse("div.text-right").unwrap();
        if let Some(price_raw) = html.select(&price_selector).next() {
            let prices: Vec<_> = price_raw
                .inner_html()
                .replace("€", "")
                .trim()
                .replace(",", ".")
                .split('/')
                .map(|p| Cents::from_euro(p.parse::<f32>().expect("Could not parse price")))
                .collect();
            Ok(MealPrice {
                student: prices[0].clone(),
                employee: prices[1].clone(),
                guest: prices[2].clone(),
            })
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
pub struct MensaCode(u16);

impl Display for MensaCode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for MensaCode {
    type Err = std::num::ParseIntError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.parse() {
            Ok(number) => Ok(MensaCode(number)),
            Err(err) => Err(err),
        }
    }
}


pub fn get_menu(mensa: &MensaCode, date: &Option<NaiveDate>) -> Result<MenuResponse, reqwest::Error> {
    match Client::new()
        .post("https://www.stw.berlin/xhr/speiseplan-wochentag.html")
        .form(&[
            ("week", "now"),
            (
                "date",
                &date.unwrap_or_else(|| Local::today().naive_local())
                    .format("%Y-%m-%d")
                    .to_string(),
            ),
            ("resources_id", &mensa.0.to_string()),
        ])
        .header(header::USER_AGENT, "Mozilla/5.0")
        .send()
    {
        Ok(mut response) => {
            assert!(response.status().is_success());
            Ok(MenuResponse::from(Html::parse_fragment(&response.text().unwrap())))
        }
        Err(e) => Err(e),
    }
}
