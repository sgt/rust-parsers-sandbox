use crate::parser::{Chumsky, Nom, ParserLib};
use clap::Parser as ClapParser;
use std::fmt::Debug;

mod ast;
mod eval;
mod parser;

#[derive(ClapParser, Debug)]
#[clap(version, about)]
struct Args {
    #[clap(value_parser)]
    filename: String,

    #[clap(short, long, value_parser)]
    parser: String,
}

fn determine_parser_lib(s: &str) -> Option<&dyn ParserLib> {
    match s {
        "chumsky" => Some(&Chumsky),
        "nom" => Some(&Nom),
        _ => None,
    }
}

fn main() {
    let args = Args::parse();
    let parser = determine_parser_lib(args.filename.as_str());
    parser
        .map(|p| p.run_file(&args.filename))
        .unwrap_or_default()
}

#[cfg(test)]
mod test {
    use crate::{
        ast::Expr,
        parser::{Chumsky, ParserLib},
    };

    #[test]
    fn number() {
        let src = " 123 ";
        let result = Chumsky.parse(src);
        assert_eq!(result.unwrap(), Expr::Num(123.0));
    }

    fn foo() {
        let mut s = String::from("zhopa");
        bar(&mut s);
    }

    fn bar(s: &mut String) {
        s.push_str(" meow")
    }
}
