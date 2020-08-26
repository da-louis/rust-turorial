use std::fs::File;
use std::io::{BufRead, BufReader, stdin};

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
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }

    println!("Is verbosity specified?: {}", opts.verbose);
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
