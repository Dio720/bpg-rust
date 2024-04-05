mod bpg;
mod cli;

use std::env;

fn welcome_msg() {
    println!(r#"
Usage: [<options>] <number_of_puzzles>
  Program options are:
      --rows <num>    Specify the number of rows for the matrix,
                          with `<num>` in the range [7, 16 ].
                          The default value is 10.
      --cols <num>    Specify the number of columns for the matrix,
                          with `<num>` in the range [7,16].
                          The default value is 10.
  Requested input is:
      number_of_puzzles    The number of puzzles to be generated,
                          in the range [1,100].
    "#)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for (index, argument) in args.iter().enumerate() {
        println!("argument {}: {}", index, argument);
    }
}
