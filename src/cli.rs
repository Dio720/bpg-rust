use clap::{value_parser, Arg, Command};

struct RunOpts {
    rows: u32,
    cols: u32,
    num_puzzles: u32,
}

pub fn parse_arguments() {
    let _args = Command::new("bpg")
        .arg(
            Arg::new("rows")
                .long("rows")
                .short('r')
                .help("Specify ther number of rows for the matrix")
                .value_parser(value_parser!(i32).range(7..=16))
                .default_value("10"), // < Default num_rows
        )
        .arg(
            Arg::new("cols")
                .long("cols")
                .short('c')
                .help("Specify the number of columns for the matrix")
                .value_parser(value_parser!(i32).range(7..=16))
                .default_value("10"), // < Default num_rows
        )
        .arg(
            Arg::new("num_puzzles")
                .required(true) // returns error when don't used
                .help("The number of puzzles to be generated")
                .value_parser(value_parser!(i32).range(7..=16)), // Converts the argument to 32-bit
            // int
        )
        .try_get_matches()
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
    
    let arg_ids = ["rows", "cols", "num_puzzles"];

    for arg in arg_ids {
        match _args.get_one::<i32>(arg) {
            Some(value) => println!("{}: {}", arg, value),
            _ => println!("{}: Doesn't exists or error", arg)
        }
    }
}
