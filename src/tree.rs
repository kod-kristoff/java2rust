use std::fmt;

#[derive(Debug)]
#[rustfmt::skip]
pub enum TreeKind {
    ErrorTree,
    File, Class, Method, TypeExpr,
    ParamList, Param,
    Block,
    StmtLet, StmtReturn, StmtExpr,
    ExprLiteral, ExprName, ExprParen,
    ExprBinary, ExprCall,
    ArgList, Arg,
}

pub struct Tree {
    pub kind: TreeKind,
    pub children: Vec<Child>,
}

pub enum Child {
    Tree(Tree),
    Token(Token),
}

use crate::{format_to, Token};
impl Tree {
    fn print(&self, buf: &mut String, level: usize) -> Result<(), fmt::Error> {
        let indent = " ".repeat(level);
        format_to!(buf, "{indent}{:?}\n", self.kind);
        for child in &self.children {
            match child {
                Child::Token(token) => {
                    format_to!(buf, "{indent}   '{}'\n", token.text);
                }
                Child::Tree(tree) => tree.print(buf, level + 1)?,
            }
        }
        assert!(buf.ends_with("\n"));
        Ok(())
    }
}
impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.print(&mut buf, 0)?;
        write!(f, "{}", buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
