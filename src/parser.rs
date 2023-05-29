use std::cell::Cell;

use crate::{tree::Child, Lexer, Token, TokenKind, Tree, TreeKind};

pub struct Parser {
    // tokens: Lexer,
    tokens: Vec<Token>,
    pos: usize,
    fuel: Cell<u32>,
    events: Vec<Event>,
}

#[derive(Debug)]
pub enum Event {
    Open { kind: TreeKind },
    Close,
    Advance,
}
pub struct MarkOpened {
    index: usize,
}

pub struct MarkClosed {
    index: usize,
}

impl Parser {
    pub fn new(tokens: Lexer) -> Self {
        Self {
            tokens: tokens.lex(),
            pos: 0,
            fuel: Cell::new(256),
            events: Vec::new(),
        }
    }

    pub fn build_tree(self) -> Tree {
        let mut tokens = self.tokens.into_iter();
        let mut events = self.events;
        let mut stack = Vec::new();

        dbg!(&events);
        assert!(matches!(events.pop(), Some(Event::Close)));

        for event in events {
            dbg!(&event);
            match event {
                Event::Open { kind } => stack.push(Tree {
                    kind,
                    children: Vec::new(),
                }),
                Event::Close => {
                    let tree = stack.pop().unwrap();
                    stack
                        .last_mut()
                        .unwrap()
                        .children
                        .push(crate::tree::Child::Tree(tree));
                }
                Event::Advance => {
                    let token = tokens.next().unwrap();
                    stack.last_mut().unwrap().children.push(Child::Token(token));
                }
            }
            dbg!(&stack);
        }

        stack.pop().unwrap()
    }

    pub fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened {
            index: self.events.len(),
        };
        self.events.push(Event::Open {
            kind: TreeKind::ErrorTree,
        });
        mark
    }

    pub fn close(&mut self, m: MarkOpened, kind: TreeKind) -> MarkClosed {
        self.events[m.index] = Event::Open { kind };
        self.events.push(Event::Close);
        MarkClosed { index: m.index }
    }

    pub fn open_before(&mut self, m: MarkClosed) -> MarkOpened {
        let mark = MarkOpened { index: m.index };
        self.events.insert(
            m.index,
            Event::Open {
                kind: TreeKind::ErrorTree,
            },
        );
        mark
    }

    pub fn advance(&mut self) {
        assert!(!self.eof());
        self.fuel.set(256);
        self.events.push(Event::Advance);
        self.pos += 1;
    }
    pub fn advance_with_error(&mut self, error: &str) {
        let m = self.open();
        // TODO: Error reporting
        eprintln!("{error}");
        self.advance();
        self.close(m, TreeKind::ErrorTree);
    }

    pub fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    pub fn nth(&self, lookahead: usize) -> TokenKind {
        if self.fuel.get() == 0 {
            panic!("parser is stuck")
        }
        self.fuel.set(self.fuel.get() - 1);
        self.tokens
            .get(self.pos + lookahead)
            .map_or(TokenKind::Eof, |it| it.kind)
    }
    pub fn at(&self, kind: TokenKind) -> bool {
        self.nth(0) == kind
    }

    pub fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }
    pub fn expect(&mut self, kind: TokenKind) {
        if self.eat(kind) {
            return;
        }
        // TODO error reporting
        eprintln!("expected {kind:?}");
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
