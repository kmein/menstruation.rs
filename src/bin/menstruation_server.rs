#![feature(proc_macro_hygiene, decl_macro)]

use menstruation::{
    codes::{self, Mensa},
    menu::{self, Meal},
    Response,
};
use rocket::{fairing::AdHoc, get, http::Header, routes};
use rocket_contrib::json::Json;

#[get("/menu?<options..>")]
fn menu(options: menu::MenuOptions) -> Option<Json<Response<Meal>>> {
    menu::get(options).map(Json).ok()
}

#[get("/codes?<pattern>")]
fn codes(pattern: Option<String>) -> Option<Json<Response<Mensa>>> {
    codes::get(pattern)
        .map(|codes_response| Json(codes_response))
        .ok()
}

fn main() {
    rocket::ignite()
        .attach(AdHoc::on_response("CORS", |_, response| {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        }))
        .mount("/", routes![menu, codes])
        .launch();
}
