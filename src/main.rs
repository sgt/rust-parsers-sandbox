use std::fs::read_to_string;

use chumsky::Parser as ChumskyParser;
use clap::Parser as ClapParser;

use crate::eval::eval;

mod ast;
mod eval;
mod parser_chumsky;

#[derive(ClapParser, Debug)]
#[clap(version, about)]
struct Args {
    #[clap(value_parser)]
    filename: String,
}

#[derive(Debug)]
enum ParserLib {
    Chumsky,
}

fn run(lib: ParserLib, src: &str) {
    let result = match lib {
        ParserLib::Chumsky => parser_chumsky::parser().parse(src),
    };

    match result {
        Ok(ast) => match eval(&ast, &mut Vec::new(), &mut Vec::new()) {
            Ok(output) => println!("{}", output),
            Err(eval_err) => println!("Evaluation error: {}", eval_err),
        },
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| println!("Parse error: {}", e)),
    }
}

fn run_file(lib: ParserLib, filename: &str) {
    run(lib, &read_to_string(filename).unwrap())
}

fn main() {
    let args = Args::parse();
    run_file(ParserLib::Chumsky, &args.filename)
}
