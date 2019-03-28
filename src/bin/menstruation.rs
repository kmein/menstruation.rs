#![feature(try_from)]

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
    Menu(menu::MenuOptions),
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    /// Lists all available dining facilities
    Codes {
        #[structopt(name = "PATTERN")]
        /// Searches for a specific pattern
        pattern: Option<String>,
    },
}

fn main() {
    let options = Options::from_args();

    match options {
        Options::Menu(menu_options) => match menu::get(menu_options) {
            Ok(menu_response) => println!("{}", menu_response),
            Err(e) => eprintln!("{}", e),
        },
        Options::Codes { pattern } => match codes::get(pattern) {
            Ok(codes_response) => println!("{}", codes_response),
            Err(e) => eprintln!("{}", e),
        },
    }
}
