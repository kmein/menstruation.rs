extern crate scraper;

use scraper::{Html, Selector};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Cents(u64);

#[derive(Debug)]
pub struct MenuResponse {
    groups: Vec<MealGroup>,
}

#[derive(Debug)]
struct MealGroup {
    name: String,
    meals: Vec<Meal>,
}

#[derive(Debug)]
struct Meal {
    name: String,
    color: MealColor,
    tags: HashSet<MealTag>,
    price: Option<MealPrice>,
    allergens: HashSet<String>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MealColor {
    Green,
    Yellow,
    Red,
}

impl From<&str> for MealColor {
    fn from(uri: &str) -> Self {
        match uri {
            "/vendor/infomax/mensen/icons/ampel_gelb_70x65.png" => MealColor::Yellow,
            "/vendor/infomax/mensen/icons/ampel_gruen_70x65.png" => MealColor::Green,
            "/vendor/infomax/mensen/icons/ampel_rot_70x65.png" => MealColor::Red,
            _ => panic!("Cannot parse URI: {}", uri),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MealTag {
    Vegetarian,
    Vegan,
    Organic,
    SustainableFishing,
    ClimateFriendly,
}

impl From<&str> for MealTag {
    fn from(uri: &str) -> Self {
        match uri {
            "/vendor/infomax/mensen/icons/1.png" => MealTag::Vegetarian,
            "/vendor/infomax/mensen/icons/15.png" => MealTag::Vegan,
            "/vendor/infomax/mensen/icons/18.png" => MealTag::Organic,
            "/vendor/infomax/mensen/icons/38.png" => MealTag::SustainableFishing,
            "/vendor/infomax/mensen/icons/43.png" => MealTag::ClimateFriendly,
            _ => panic!("Canno parse URI: {}", uri),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct MealPrice {
    student: Cents,
    employee: Cents,
    guest: Cents,
}

fn partition<A, P, I>(predicate: P, xs: I) -> (Vec<A>, Vec<A>)
where
    P: Fn(&A) -> bool,
    I: Iterator<Item = A>,
{
    let mut toepfchen = Vec::new();
    let mut kroepfchen = Vec::new();
    for x in xs {
        if predicate(&x) {
            toepfchen.push(x);
        } else {
            kroepfchen.push(x);
        }
    }
    (toepfchen, kroepfchen)
}

pub fn extract_response(html: &str) -> MenuResponse {
    let groups_selector = Selector::parse(".splGroupWrapper").unwrap();
    let group_name_selector = Selector::parse(".splGroup").unwrap();
    let meal_selector = Selector::parse(".splMeal").unwrap();
    let icon_selector = Selector::parse("img.splIcon").unwrap();
    let meal_name_selector = Selector::parse("span.bold").unwrap();
    let price_selector = Selector::parse("div.text-right").unwrap();

    let dom = Html::parse_fragment(html);

    println!("{:?}", dom);

    let mut groups = Vec::new();
    for group_html in dom.select(&groups_selector) {
        let group_name = group_html
            .select(&group_name_selector)
            .next()
            .unwrap()
            .inner_html();
        let mut meals = Vec::new();
        for meal_html in group_html.select(&meal_selector) {
            let icons_html = meal_html.select(&icon_selector).map(|img| {
                img.value().attr("src").unwrap()
            });
            let (color_htmls, tag_htmls) = partition(|&src| src.contains("ampel"), icons_html);
            let color = MealColor::from(color_htmls[0]);
            let tags = tag_htmls.iter().map(|&src| MealTag::from(src)).collect();
            let meal_name = meal_html
                .select(&meal_name_selector)
                .next()
                .unwrap()
                .inner_html();
            let price = if let Some(price_raw) = meal_html.select(&price_selector).next() {
                let prices: Vec<_> = price_raw
                    .inner_html()
                    .replace("€ ", "")
                    .replace(",", ".")
                    .split("/")
                    .map(|p| Cents((p.parse::<f64>().unwrap() * 100f64) as u64))
                    .collect();
                Some(MealPrice {
                    student: prices[0].clone(),
                    employee: prices[1].clone(),
                    guest: prices[2].clone(),
                })
            } else {
                None
            };
            // allergens here
            meals.push(Meal {
                name: meal_name,
                tags,
                color,
                price,
                allergens: HashSet::new(),
            });
        }
        groups.push(MealGroup {
            name: group_name,
            meals,
        });
    }
    MenuResponse { groups }
}
