mod tree;
pub use tree::{Child, Tree, TreeKind};

mod lexer;
use lexer::Lexer;

mod parser;
use parser::Parser;

mod token;
pub use token::{Token, TokenKind};
mod rules;

pub fn parse_tree(text: &str) -> Tree {
    println!("parse_tree");
    let tokens = Lexer::new(text);
    let mut p = Parser::new(tokens);
    rules::file(&mut p);
    p.build_tree()
}
