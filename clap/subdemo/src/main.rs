extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("DemoApp")
        .subcommand(
            SubCommand::with_name("install")
                .about("controls install features")
                .version("1.2")
                .author("Chaos Mo")
                .arg(
                    Arg::with_name("force")
                        .short("f")
                        .long("force")
                        .help("install with force arg"),
                ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("install") {
        if matches.is_present("force") {
            println!("force is not right...");
        } else {
            println!("general install...");
        }
    }
}
