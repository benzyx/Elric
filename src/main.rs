mod event;
mod exchange;
mod orderbook;
mod types;

use crate::exchange::*;

extern crate clap;
use clap::{App, Arg};

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let matches = App::new("Elric")
        .version("1.0")
        .author("Ben Zhang <frenzybenzy@gmail.com>")
        .about("Rust trading platform")
        .arg(
            Arg::with_name("SYMBOLS")
                .short("s")
                .long("symbols")
                .help("Comma separated string containing all valid symbols")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CONFIG")
                .short("c")
                .long("config")
                .help("Sets config file")
                .takes_value(true),
        )
        .get_matches();

    let config = matches.value_of("CONFIG").unwrap_or("default.conf");
    let symbols = String::from(matches.value_of("SYMBOLS").unwrap());

    let symbols_vec: Vec<String> = symbols.split(',').map(|s| s.to_string()).collect();
    let exch = Exchange::new(Some(symbols_vec));
}
