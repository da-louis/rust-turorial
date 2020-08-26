use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
name = "My RPN program :)",
version = "1.0.0",
author = "da-louis",
about = "Super awesome fantastic happy great highest sample RPN calculator :doge:"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        run(reader, opts.verbose)
    } else {
        println!("No file specified")
    }

    println!("Is verbosity specified?: {}", opts.verbose);
}

fn run(reader: BufReader<File>, verbose: bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
