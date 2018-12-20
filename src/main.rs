#![feature(try_from)]
extern crate chrono;
extern crate menstruation;
extern crate reqwest;
extern crate structopt;

use menstruation::*;
use chrono::{Local, NaiveDate, ParseError};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    rename_all = "kebab-case",
    raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
struct Options {
    /// Display only meals with the specified colors
    #[structopt(short, long, parse(try_from_str))]
    colors: Vec<MealColor>,

    /// Display only meals with the specified tags
    #[structopt(short, long, parse(try_from_str))]
    tags: Vec<MealTag>,

    /// Display no meals more expensive than a given price
    #[structopt(short = "p", long)]
    max_price: Option<Cents>,

    /// Display no meals containing the specified allergens
    #[structopt(short, long)]
    allergens: Vec<String>,

    /// Display a given date's menu
    #[structopt(short, long, parse(try_from_str = "parse_iso_date"))]
    date: Option<NaiveDate>,

    /// Display a given mensa's menu
    #[structopt(short, long, default_value = "191")]
    mensa: MensaCode,
}

fn parse_iso_date(string: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(string, "%Y-%m-%d")
}

fn filter_menu(options: &Options, menu: MenuResponse) -> MenuResponse {
    fn matches_query(options: &Options, meal: &Meal) -> bool {
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
                    || (tag == &MealTag::Vegan && options.tags.contains(&MealTag::Vegetarian))
            });
        let allergens_ok = meal
            .allergens
            .iter()
            .all(|allergen| !options.allergens.contains(allergen));
        price_ok && colors_ok && tags_ok && allergens_ok
    }

    let mut groups = Vec::new();
    for group in menu.0 {
        let meals = group
            .meals
            .into_iter()
            .filter(|meal| matches_query(options, meal))
            .collect::<Vec<_>>();
        if !meals.is_empty() {
            groups.push(MealGroup { meals, ..group });
        }
    }
    MenuResponse(groups)
}

fn main() {
    let options = Options::from_args();

    if let Ok(menu_response) = get_menu(&options.mensa, &options.date) {
        println!("{}", filter_menu(&options, menu_response));
    } else {
        eprintln!(
            "Kein Speiseplan gefunden für Mensa {} am {}.",
            options.mensa,
            options
                .date
                .unwrap_or_else(|| Local::today().naive_local())
                .format("%d.%m.%Y")
        );
    }
}
