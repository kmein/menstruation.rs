#![feature(try_from)]
extern crate reqwest;
extern crate chrono;
mod lib;

struct MenuQuery {
    max_price: Option<lib::Cents>,
    allowed_colors: Option<Vec<lib::MealColor>>,
    allowed_tags: Option<Vec<lib::MealTag>>,
}

struct MensaCode(u16);

fn menu_html(
    date: chrono::Date<chrono::Local>,
    mensa_code: MensaCode,
) -> reqwest::Result<reqwest::Response> {
    reqwest::Client::new()
        .post("https://www.stw.berlin/xhr/speiseplan-wochentag.html")
        .form(
            &[
                ("week", "now"),
                ("date", &date.format("%Y-%m-%d").to_string()),
                ("resources_id", &mensa_code.0.to_string()),
            ],
        )
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0")
        .send()
}

fn main() {
    let mut response = menu_html(
        chrono::Local::today(),
        MensaCode(191),
    ).unwrap();
    assert!(response.status().is_success());

    let menu_response = lib::extract(&response.text().unwrap());

    println!("{}", menu_response);
}
