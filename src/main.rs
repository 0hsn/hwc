mod counter;

use clap::{App, Arg, ArgMatches};
use counter::{ArgDefinition, CounterOps};

fn get_matches(arg_def: &ArgDefinition) -> ArgMatches {
    App::new("word counter")
        .version("0.1.0")
        .author("Mahmudul Hasan <hasanlock@gmail.com>")
        .about("wc command replacement for good! support's multi-byte chars too.")
        .arg(
            Arg::with_name(arg_def.byte.1)
                .short(arg_def.byte.0.to_string())
                .takes_value(false)
                .required(false)
                .help(arg_def.byte.2),
        )
        .arg(
            Arg::with_name(arg_def.char.1)
                .short(arg_def.char.0.to_string())
                .takes_value(false)
                .required(false)
                .help(arg_def.char.2),
        )
        .arg(
            Arg::with_name(arg_def.line.1)
                .short(arg_def.line.0.to_string())
                .takes_value(false)

                .required(false)
                .help(arg_def.line.2),
        )
        .arg(
            Arg::with_name(arg_def.word.1)
                .short(arg_def.word.0.to_string())
                .takes_value(false)
                .required(false)
                .help(arg_def.word.2),
        )
        .arg(
            Arg::with_name(arg_def.file.1)
                .takes_value(true)
                .required(true)
                .help(arg_def.file.2),
        )
        .get_matches()
}

fn main() {
    let arg_def: ArgDefinition = Default::default();
    let matches = get_matches(&arg_def);
    let ops = CounterOps::from_clap_arg_matches(&matches, &arg_def);
    let result = ops.calculate();
    println!("{}", result);
}
