use clap::{App, Arg};

struct RunOpts {
    rows: u32,
    cols: u32,
    num_puzzles: u32,
}

static HELP_MESSAGE: &str = r#"    
    Usage: [<options>] <number_of_puzzles>
        Program options are:
            --rows <num>	Specify the number of rows for the matrix,
                                with `<num>` in the range [7, 16 ].
                                The default value is 10.
            --cols <num>	Specify the number of columns for the matrix,
                                with `<num>` in the range [7,16].
                                The default value is 10.
        Requested input is:
            number_of_puzzles	The number of puzzles to be generated,
                                in the range [1,100]."#;

pub fn parse_arguments() -> RunOpts {
    let arguments = App::new("bpg")
        .author("Vinicius Lima")
        .about("Generate from [0,100] battleship game puzzles with n×n dimensions")
        .arg(Arg::with_name("rows")
            .long("rows")
            .short('r')
            .takes_value(true)
            .help("Specify ther number of rows for the matrix")
            .value_parser(validate_dimension))
        .arg(Arg::with_name("cols")
            .long("cols")
            .short('c')
            .takes_value(true)
            .help("Specify the number of columns for the matrix")
            .value_parser(validate_dimension))
    
}

