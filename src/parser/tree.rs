use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug)]
pub enum Child {
    Tree(Tree),
    Token(Token),
}

use crate::{format_to, parser::Token, parser::TokenKind};
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

    pub fn kind(&self) -> TreeKind {
        self.kind
    }

    pub fn child_by_token(&self, token: TokenKind) -> Option<&Token> {
        self.children.iter().find_map(|c| match c {
            Child::Token(t) if t.kind == token => Some(t),
            _ => None,
        })
    }

    pub fn child_by_tree(&self, kind: TreeKind) -> Option<&Tree> {
        self.children.iter().find_map(move |c| match c {
            Child::Tree(t) if t.kind == kind => Some(t),
            _ => None,
        })
    }

    pub fn children_by_tree(&self, kind: TreeKind) -> impl Iterator<Item = &Tree> {
        self.children.iter().filter_map(move |c| match c {
            Child::Tree(t) if t.kind == kind => Some(t),
            _ => None,
        })
    }

    pub fn child_value_by_token(&self, kind: TokenKind) -> Option<&str> {
        self.child_by_token(kind).map(|token| token.text.as_str())
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
