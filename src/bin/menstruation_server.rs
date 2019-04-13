#![feature(proc_macro_hygiene, decl_macro)]

use menstruation::{
    allergens::{self, Allergen},
    codes::{self, Mensa},
    menu::{self, Meal},
    Group, Response,
};
use rocket::{fairing::AdHoc, get, http::Header, routes};
use rocket_contrib::json::Json;

#[get("/menu?<options..>")]
fn menu(options: menu::MenuOptions) -> Option<Json<Response<Meal>>> {
    menu::get(options).map(Json).ok()
}

#[get("/codes?<pattern>")]
fn codes(pattern: Option<String>) -> Option<Json<Response<Mensa>>> {
    codes::get(pattern).map(Json).ok()
}

#[get("/allergens")]
fn allergens() -> Option<Json<Group<Allergen>>> {
    allergens::get().map(Json).ok()
}

fn main() {
    rocket::ignite()
        .attach(AdHoc::on_response("CORS", |_, response| {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        }))
        .mount("/", routes![menu, codes, allergens])
        .launch();
}
