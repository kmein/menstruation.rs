extern crate reqwest;
mod lib;

struct MenuQuery {
    max_price: Option<lib::Cents>,
    allowed_colors: Option<Vec<lib::MealColor>>,
    allowed_tags: Option<Vec<lib::MealTag>>,
}

struct MensaCode(u16);

fn main() {
    let mensa_code = MensaCode(191);
    let params = [
        ("week", "now"),
        ("date", "2018-12-18"),
        ("resources_id", &mensa_code.0.to_string()),
    ];
    let mut response = reqwest::Client::new()
        .post("https://www.stw.berlin/xhr/speiseplan-wochentag.html")
        .form(&params)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0")
        .send().unwrap();
    assert!(response.status().is_success());

    println!("{:?}", response.text().unwrap());

    let menu_response = lib::extract_response(&response.text().unwrap());

    println!("{:?}", menu_response);
}
