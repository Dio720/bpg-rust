use clap::{value_parser, Arg, Command};

pub struct RunOpts {
    pub rows: i32,
    pub cols: i32,
    pub num_puzzles: i32,
}

impl RunOpts {
    pub fn new(rows: i32, cols: i32, num_puzzles: i32) -> Self {
        Self {
            rows,
            cols,
            num_puzzles,
        }
    }
}

pub fn parse_arguments() -> RunOpts {
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
                .value_parser(value_parser!(i32).range(0..=100)), // Converts the argument to 32-bit
                                                                 // int
        )
        .try_get_matches()
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });

    let arg_ids = ["rows", "cols", "num_puzzles"];

    let mut run_opts = RunOpts::new(10, 10, 10);

    for arg in arg_ids {
        match _args.get_one::<i32>(arg) {
            Some(value) => match arg {
                "rows" => run_opts.rows = *value,
                "cols" => run_opts.cols = *value,
                "num_puzzles" => run_opts.num_puzzles = *value,
                _ => unreachable!(),
            },
            None => {
                eprintln!("Error: Argument {} not found", arg);
                std::process::exit(1);
            }
        }
    }
    return run_opts;
}
