extern crate scraper;
extern crate regex;
extern crate ansi_term;

use std::fmt;
use std::collections;


mod utility {
    pub fn partition<A, P, I>(predicate: P, xs: I) -> (Vec<A>, Vec<A>)
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
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Cents(u64);

impl Cents {
    fn from_euro(euro: f32) -> Self {
        Cents((euro * 100f32) as u64)
    }
}

impl fmt::Display for Cents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Cents(total_cents) = self;
        let euros = total_cents / 100;
        let cents = total_cents % 100;
        write!(f, "{},{:02} €", euros, cents)
    }
}

#[derive(Debug)]
pub struct MenuResponse {
    groups: Vec<MealGroup>,
}

impl fmt::Display for MenuResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for group in &self.groups {
            write!(f, "{}", group);
        }
        Ok(())
    }
}

#[derive(Debug)]
struct MealGroup {
    name: String,
    meals: Vec<Meal>,
}

impl fmt::Display for MealGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}",
            ansi_term::Style::new().bold().paint(
                &self.name.to_uppercase(),
            )
        );
        for meal in &self.meals {
            write!(f, "{}", meal);
        }
        writeln!(f, "");
        Ok(())
    }
}

#[derive(Debug)]
struct Meal {
    name: String,
    color: MealColor,
    tags: collections::HashSet<MealTag>,
    price: Option<MealPrice>,
    allergens: collections::HashSet<String>,
}

impl fmt::Display for Meal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn to_ansi(color: &MealColor) -> ansi_term::Colour {
            match color {
                MealColor::Green => ansi_term::Colour::Green,
                MealColor::Red => ansi_term::Colour::Red,
                MealColor::Yellow => ansi_term::Colour::Yellow,
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

impl fmt::Display for MealTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            ansi_term::Style::new().italic().paint(match self {
                MealTag::Vegetarian => "vegetarisch",
                MealTag::Vegan => "vegan",
                MealTag::Organic => "bio",
                MealTag::SustainableFishing => "nachhaltig",
                MealTag::ClimateFriendly => "öko",
            })
        )
    }
}

impl From<&str> for MealTag {
    fn from(uri: &str) -> Self {
        match uri {
            "/vendor/infomax/mensen/icons/1.png" => MealTag::Vegetarian,
            "/vendor/infomax/mensen/icons/15.png" => MealTag::Vegan,
            "/vendor/infomax/mensen/icons/18.png" => MealTag::Organic,
            "/vendor/infomax/mensen/icons/38.png" => MealTag::SustainableFishing,
            "/vendor/infomax/mensen/icons/43.png" => MealTag::ClimateFriendly,
            _ => panic!("Cannot parse URI: {}", uri),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct MealPrice {
    student: Cents,
    employee: Cents,
    guest: Cents,
}

pub fn extract_response(html: &str) -> Result<MenuResponse, &str> {
    let groups_selector = scraper::Selector::parse(".splGroupWrapper").unwrap();
    let group_name_selector = scraper::Selector::parse(".splGroup").unwrap();
    let meal_selector = scraper::Selector::parse(".splMeal").unwrap();
    let icon_selector = scraper::Selector::parse("img.splIcon").unwrap();
    let meal_name_selector = scraper::Selector::parse("span.bold").unwrap();
    let price_selector = scraper::Selector::parse("div.text-right").unwrap();
    let allergen_selector = scraper::Selector::parse(".toolt").unwrap();

    let dom = scraper::Html::parse_fragment(html);

    let mut groups = Vec::new();
    for group_html in dom.select(&groups_selector) {
        let group_name = group_html
            .select(&group_name_selector)
            .next()
            .expect("No group name found")
            .inner_html();
        let mut meals = Vec::new();
        for meal_html in group_html.select(&meal_selector) {
            let icons_html = meal_html.select(&icon_selector).map(|img| {
                img.value().attr("src").expect("Icon has no src")
            });
            let (color_htmls, tag_htmls) =
                utility::partition(|&src| src.contains("ampel"), icons_html);
            let color = MealColor::from(color_htmls[0]);
            let tags = tag_htmls.iter().map(|&src| MealTag::from(src)).collect();
            let meal_name = meal_html
                .select(&meal_name_selector)
                .next()
                .expect("No meal name found")
                .inner_html()
                .trim()
                .to_string();
            let price = if let Some(price_raw) = meal_html.select(&price_selector).next() {
                let prices: Vec<_> = price_raw
                    .inner_html()
                    .replace("€", "")
                    .trim()
                    .replace(",", ".")
                    .split("/")
                    .map(|p| {
                        Cents::from_euro(p.parse::<f32>().expect("Could not parse price"))
                    })
                    .collect();
                Some(MealPrice {
                    student: prices[0].clone(),
                    employee: prices[1].clone(),
                    guest: prices[2].clone(),
                })
            } else {
                None
            };
            let allergens = {
                let parenthesized = regex::Regex::new(r"\((.*)\)").unwrap();
                let allergens_html = meal_html
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
                    collections::HashSet::new()
                }
            };
            meals.push(Meal {
                name: meal_name,
                tags,
                color,
                price,
                allergens,
            });
        }
        groups.push(MealGroup {
            name: group_name,
            meals,
        });
    }
    Ok(MenuResponse { groups })
}
