use clap::{value_parser, Arg, Command};

// u16 stands for unsigned int 16-bit
type Nat = u16;

pub struct Config {
    pub rows: Nat,
    pub cols: Nat,
    pub num_puzzles: Nat,
}

impl Config {
    pub fn new(rows: Nat, cols: Nat, num_puzzles: Nat) -> Self {
        Self {
            rows,
            cols,
            num_puzzles,
        }
    }
}

pub fn parse_arguments() -> Config {
    let _args = Command::new("bpg")
        .arg(
            Arg::new("rows")
                .long("rows")
                .short('r')
                .help("Specify ther number of rows for the matrix")
                .value_parser(value_parser!(Nat).range(7..=16))
                .default_value("10"), // < Default num_rows
        )
        .arg(
            Arg::new("cols")
                .long("cols")
                .short('c')
                .help("Specify the number of columns for the matrix")
                .value_parser(value_parser!(Nat).range(7..=16))
                .default_value("10"), // < Default num_rows
        )
        .arg(
            Arg::new("num_puzzles")
                .required(true) // returns error when don't used
                .help("The number of puzzles to be generated")
                .value_parser(value_parser!(Nat).range(0..=100)), // Converts the argument to 32-bit
                                                                 // int
        )
        .try_get_matches()
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });

    let arg_ids = ["rows", "cols", "num_puzzles"];

    let mut run_opts = Config::new(10, 10, 10);

    for arg in arg_ids {
        match _args.get_one::<Nat>(arg) {
            Some(value) => match arg {
                "rows"        => run_opts.rows = *value,
                "cols"        => run_opts.cols = *value,
                "num_puzzles" => run_opts.num_puzzles = *value,
                _             => unreachable!(),
            },
            None => {
                eprintln!("Error: Argument {} not found", arg);
                std::process::exit(1);
            }
        }
    }
    return run_opts;
}
