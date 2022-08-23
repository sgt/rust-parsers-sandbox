use super::ParserLib;

pub struct Nom;

impl ParserLib for Nom {
    fn parse(&self, src: &str) -> Result<crate::ast::Expr, Vec<String>> {
        todo!()
    }
}
