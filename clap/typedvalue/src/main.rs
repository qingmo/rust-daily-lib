extern crate clap;

use clap::{App, value_t, values_t};

fn main() {
    let matches = App::new("myapp")
        // 等价于
        // .args(
        //     &[
        //         Arg::with_name("seq")
        //         .multiple(true)
        //         .help("A sequence of whole positive numbers, i.e. 20 25 30"),
        //         Arg::with_name("len")
        //         .short("l")
        //         .long("len")
        //         .value_name("len")
        //         .help("A length to use, defaults to 10 when omitted")
        //     ]
        // )
        .args_from_usage(
            "<seq>... 'A sequence of whole positive numbers, i.e. 20 25 30'
            --len -l [len] 'A length to use, defaults to 10 when omitted'
            --offline -o [isoffline] 'is computer offline'",
        )
        .get_matches();

    let len: u32 = value_t!(matches, "len", u32).unwrap_or(10);
    println!("len ({}) + 2 = {}", len, len + 2);
    let online: bool = value_t!(matches, "offline", bool).unwrap_or(false);
    if online {
        println!("computer is online");
    } else {
        println!("computer is offline");
    }
    // 如果类型不满足，可以中断程序退出
    for v in values_t!(matches, "seq", u32).unwrap_or_else(|e| e.exit()) {
        println!("Sequence part {} + 2: {}", v, v + 2);
    }
}
