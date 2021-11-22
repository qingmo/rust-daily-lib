#[macro_use]
extern crate clap;
use clap::App;

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    if let Some(matches) = matches.subcommand_matches("install") {
        if matches.is_present("force") {
            println!("force is not right...");
        } else {
            println!("general install...");
        }
    }
}
