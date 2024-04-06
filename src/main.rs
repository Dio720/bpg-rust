use cli::parse_arguments;

mod bpg;
mod cli;

fn main() {
    let run_opts = parse_arguments();

    println!("Rows: {}", run_opts.rows);
    println!("Cols: {}", run_opts.cols);
    println!("Num Puzzles: {}", run_opts.num_puzzles);
}
