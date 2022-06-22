use clap;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(clap::Parser, Default, Debug)]
pub struct Cli {
    /// The pattern to look for
    #[clap(short = 'N', value_parser, default_value_t = 1000)]
    pub number: i32,
}