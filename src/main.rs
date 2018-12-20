#![feature(try_from)]
extern crate chrono;
extern crate menstruation;
extern crate reqwest;
extern crate structopt;

use chrono::{NaiveDate, ParseError};
use menstruation::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    rename_all = "kebab-case",
    raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
enum Options {
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Displays the menu
    Menu(MenuOptions),
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Lists all available dining facilities
    Codes {
        #[structopt(name = "PATTERN")]
        /// Searches for a specific pattern
        pattern: Option<String>,
    },
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct MenuOptions {
    #[structopt(short, long, parse(try_from_str))]
    /// Displays only meals with the specified colors
    colors: Vec<menu::Color>,
    #[structopt(short, long, parse(try_from_str))]
    /// Displays only meals with the specified tags
    tags: Vec<menu::Tag>,
    #[structopt(short = "p", long)]
    /// Displays no meals more expensive than a given price
    max_price: Option<menu::Cents>,
    #[structopt(short, long)]
    /// Displays no meals containing the specified allergens
    allergens: Vec<String>,
    #[structopt(short, long, parse(try_from_str = "parse_iso_date"))]
    /// Chooses the menu date
    date: Option<NaiveDate>,
    #[structopt(short, long, default_value = "191")]
    /// Chooses a dining facility
    mensa: MensaCode,
}

fn parse_iso_date(string: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(string, "%Y-%m-%d")
}

fn query(options: &MenuOptions, menu: Response<menu::Meal>) -> Response<menu::Meal> {
    filter_response(menu, |meal| {
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
                    || (tag == &menu::Tag::Vegan && options.tags.contains(&menu::Tag::Vegetarian))
            });
        let allergens_ok = meal
            .allergens
            .iter()
            .all(|allergen| !options.allergens.contains(allergen));
        price_ok && colors_ok && tags_ok && allergens_ok
    })
}

fn main() {
    let options = Options::from_args();

    match options {
        Options::Menu(menu_options) => match menu::get(&menu_options.mensa, menu_options.date) {
            Ok(menu_response) => println!("{}", query(&menu_options, menu_response)),
            Err(e) => eprintln!("{}", e),
        },
        Options::Codes { pattern } => match codes::get() {
            Ok(codes_response) => println!(
                "{}",
                match pattern {
                    Some(p) => filter_response(codes_response, |mensa| mensa
                        .name
                        .to_lowercase()
                        .contains(&p.to_lowercase())),
                    None => codes_response,
                }
            ),
            Err(e) => eprintln!("{}", e),
        },
    }
}
