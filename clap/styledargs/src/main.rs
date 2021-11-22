extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("DemoApp")
        .version("1.0")
        .author("Chaos Mo")
        .about("Demo App About Information")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("demo arg for cmd"),
        )
        .arg(
            Arg::with_name("optimize")
                .short("o")
                .long("optimize")
                .multiple(true)
                .help("Sets the level of optimize"),
        )
        .get_matches();
    if let Some(c) = matches.value_of("config") {
        println!("value of config: {}", c);
    }

    match matches.occurrences_of("optimize") {
        0 => println!("No optimize"),
        1 => println!("Basic optimize"),
        2 => println!("Better optimize"),
        3 | _ => println!("Are you kidding me?"),
    }
}
