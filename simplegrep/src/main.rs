use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod ast;
mod automaton;
mod parser;
mod tokens;

#[derive(Parser)]
#[command(name = "simplegrep")]
#[command(about = "A custom regular expression engine with grep-like CLI")]
struct Cli {
    #[arg(short, long)]
    pattern: String,

    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long)]
    invert_match: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Parse the regular expression
    let mut parser = parser::Parser::new(&cli.pattern);
    let regex_ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Error parsing regex: {}", e);
            std::process::exit(1);
        }
    };

    // Build NFA from AST
    let nfa = automaton::NFA::from_regex(&regex_ast);

    // Read input
    let input: Box<dyn BufRead> = if let Some(filename) = &cli.file {
        Box::new(BufReader::new(File::open(filename)?))
    } else {
        Box::new(io::stdin().lock())
    };

    // Process lines
    for (line_num, line) in input.lines().enumerate() {
        let line = line?;
        let matches = nfa.matches(&line);

        let should_print = if cli.invert_match { !matches } else { matches };

        if should_print {
            if cli.file.is_some() {
                println!("{}:{}", cli.file.as_ref().unwrap(), line_num + 1);
            }
            println!("{}", line);
        }
    }

    Ok(())
}
