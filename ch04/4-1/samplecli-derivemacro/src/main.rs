use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,

    /// Formulas written in RPN
    #[clap(name = "DIF")]
    formula_dif: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    match opts.formula_file {
        Some(sth) => println!("File specified: {}", sth),
        None => println!("No file specified."),
    }
    match opts.formula_dif {
        Some(sth) => println!("Dif: {}", sth),
        None => println!("No in Dif."),
    }

    println!("Is verbosity specified?: {}", opts.verbose);
}
