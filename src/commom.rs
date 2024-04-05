// DSIZE stands for Dimension size
pub const MAX_DSIZE: usize = 16;
pub const MIN_DSIZE: usize = 7;

pub const MAX_N_PUZZLES: usize = 10;

pub const DEFAULT_N_PUZZLES: usize = 1;
pub const DEFAULT: usize = 10;

pub struct RunningOpts {
    pub num_puzzles: usize,
    pub num_rows: usize,
    pub num_cols: usize,
    // TODO: Default output file
}

// Constructor
impl RunningOpts {
    pub fn new() -> Self {
        Self {
            num_puzzles: DEFAULT_N_PUZZLES,
            num_rows: DEFAULT,
            num_cols: DEFAULT,
        }
    }
}
