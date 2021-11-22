extern crate clap;

use clap::{App, Arg};

fn main() {
    let matched = App::new("MyApp")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("demo arg for cmd")
        )
        .get_matches();
    if let Some(c) = matched.value_of("config") {
        println!("value of config: {}", c);
    }

    println!("hello");
}
