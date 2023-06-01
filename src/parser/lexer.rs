use crate::parser::Token;
use crate::parser::TokenKind::*;

pub struct Lexer<'text> {
    text: &'text str,
}

impl<'text> Lexer<'text> {
    pub fn new(text: &'text str) -> Self {
        Self { text }
    }

    pub fn lex(self) -> Vec<Token> {
        let punctuation = (
            "( ) { } = ; , : -> + - * /",
            [
                LParen, RParen, LCurly, RCurly, Eq, Semi, Comma, Colon, Arrow, Plus, Minus, Star,
                Slash,
            ],
        );

        let keywords = (
            "class return true false",
            [ClassKeyword, ReturnKeyword, TrueKeyword, FalseKeyword],
        );
        let mut text = self.text;

        let mut result = Vec::new();
        while !text.is_empty() {
            if let Some(rest) = trim(text, |it| it.is_ascii_whitespace()) {
                text = rest;
                continue;
            }
            // dbg!(&text);
            let text_orig = text;
            let mut kind = 'kind: {
                for (i, symbol) in punctuation.0.split_ascii_whitespace().enumerate() {
                    if let Some(rest) = text.strip_prefix(symbol) {
                        text = rest;
                        break 'kind punctuation.1[i];
                    }
                }
                if let Some(rest) = trim(text, name_char) {
                    text = rest;
                    break 'kind Name;
                }
                // if let Some(rest) = trim(text) {
                //     text = rest;
                //     break 'kind JString;
                // }
                let error_index = text
                    .find(|it: char| it.is_ascii_whitespace())
                    .unwrap_or(text.len());
                text = &text[error_index..];
                ErrorToken
            };
            // dbg!(&kind);
            assert!(text.len() < text_orig.len());
            let token_text = &text_orig[..text_orig.len() - text.len()];
            if kind == Name {
                for (i, symbol) in keywords.0.split_ascii_whitespace().enumerate() {
                    if token_text == symbol {
                        kind = keywords.1[i];
                        break;
                    }
                }
            }
            result.push(Token {
                kind,
                text: token_text.to_string(),
            });
        }

        // dbg!(&result);
        return result;
    }
}

fn name_char(c: char) -> bool {
    matches!(c, '_' | 'a'..='z' | 'A'..='Z' | '0'..='9')
}

pub fn trim(text: &str, predicate: impl std::ops::Fn(char) -> bool) -> Option<&str> {
    let index = text.find(|it: char| !predicate(it)).unwrap_or(text.len());
    if index == 0 {
        None
    } else {
        Some(&text[index..])
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
