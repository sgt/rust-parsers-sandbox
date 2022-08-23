use std::fs::read_to_string;

pub use self::chumsky::Chumsky;
pub use self::nom::Nom;
use crate::{ast::Expr, eval};

pub mod chumsky;
pub mod nom;

pub trait ParserLib {
    fn parse(&self, src: &str) -> Result<Expr, Vec<String>>;

    fn run(&self, src: &str) {
        match self.parse(src) {
            Ok(ast) => match eval::eval(&ast, &mut Vec::new(), &mut Vec::new()) {
                Ok(output) => println!("{}", output),
                Err(eval_err) => println!("Evaluation error: {}", eval_err),
            },
            Err(parse_errs) => parse_errs
                .into_iter()
                .for_each(|e| println!("Parse error: {}", e)),
        }
    }

    fn run_file(&self, filename: &str) {
        self.run(&read_to_string(filename).unwrap())
    }
}
