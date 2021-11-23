#[macro_use]
extern crate clap;

use clap::{App, Arg};

fn main() {
    let app = construct_app();
    let matches = app.get_matches();
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);
    match matches.occurrences_of("optimize") {
        0 => println!("No optimize"),
        1 => println!("Basic optimize"),
        2 => println!("Better optimize"),
        3 | _ => println!("Are you kidding me?"),
    }
    println!("hello");
}

fn construct_app() -> App<'static, 'static> {
    App::new("testclap")
        .about("about test clap information")
        .author("Chaos Mo")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("FILE")
                .help("config information"),
        )
        .arg(
            Arg::with_name("optimize")
                .short("o")
                .long("optimize")
                .multiple(true)
                .help("optimize level"),
        )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let arg_vec = vec!["--", "-oo", "-c", "haha"];
        let matches = construct_app().get_matches_from(arg_vec); 
        assert!(matches.is_present("optimize"));
        assert_eq!(matches.occurrences_of("optimize"), 2);
        assert_eq!(matches.value_of("config"), Some("haha"));
    }
}