mod counter;

use clap::{App, Arg, ArgMatches};
use counter::CounterOps;

fn get_matches() -> ArgMatches<'static> {
    App::new("word counter")
        .version("0.1.0")
        .author("Mahmudul Hasan <hasanlock@gmail.com>")
        .about("wc command replacement for good! support's multi-byte chars too.")
        .arg(
            Arg::with_name("byte")
                .short("b")
                .takes_value(false)
                .required(false)
                .help("Prints number of bytes"),
        )
        .arg(
            Arg::with_name("char")
                .short("c")
                .takes_value(false)
                .required(false)
                .help("Prints number of characters"),
        )
        .arg(
            Arg::with_name("line")
                .short("l")
                .takes_value(false)
                .required(false)
                .help("Prints number of lines"),
        )
        .arg(
            Arg::with_name("word")
                .short("w")
                .takes_value(false)
                .required(false)
                .help("Prints number of words"),
        )
        .arg(
            Arg::with_name("FILE")
                .takes_value(true)
                .required(true)
                .help("Filename to check from"),
        )
        .get_matches()
}

fn main() {
    let matches = get_matches();
    let ops = CounterOps::from_clap_arg_matches(&matches);
    ops.display();
}
