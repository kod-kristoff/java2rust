use crate::parser::parser::MarkClosed;
use crate::parser::parser::Parser;
use crate::parser::TokenKind;
use crate::parser::TokenKind::*;
use crate::parser::TreeKind::*;

use super::ParseResult;

pub fn file(p: &mut Parser) -> ParseResult<()> {
    println!("rules.file");
    let m = p.open();
    // dbg!(p.eof());
    while !p.eof() {
        // dbg!(p.nth(0));
        if p.at(ClassKeyword) {
            class(p)?;
        } else {
            p.advance_with_error("expected an object");
        }
    }
    p.close(m, File);
    Ok(())
}

fn class(p: &mut Parser) -> ParseResult<()> {
    assert!(p.at(ClassKeyword));
    let m = p.open();
    p.expect(ClassKeyword)?;
    p.expect(Name)?;
    p.expect(LCurly)?;
    while !p.at(RCurly) && !p.eof() {
        if method(p)? {
            println!("found method");
        } else {
            p.advance_with_error("expected a method");
        }
    }
    p.expect(RCurly)?;
    p.close(m, Class);
    Ok(())
}

// Fn = 'name' 'name' ParamList ('->' TypeExpr)? Block
fn method(p: &mut Parser) -> ParseResult<bool> {
    // assert!(p.at(ClassKeyword));
    if p.nth(0) == Name && p.nth(1) == Name && p.nth(2) == LParen {
        let m = p.open();
        type_expr(p)?;
        p.expect(Name)?;

        if p.at(LParen) {
            param_list(p)?;
        }

        if p.at(LCurly) {
            block(p)?;
        }
        p.close(m, Method);
        return Ok(true);
    }
    // let m = p.open();
    // p.expect(ClassKeyword);
    // p.expect(Name);
    // p.expect(LCurly);
    // while !p.at(RCurly) {
    //     method(p);
    // }
    // p.expect(RCurly);
    // p.close(m, Class);
    Ok(false)
}

// ParamList = '(' Param* ')'
fn param_list(p: &mut Parser) -> ParseResult<()> {
    assert!(p.at(LParen));
    let m = p.open();

    p.expect(LParen)?;
    while !p.at(RParen) && !p.eof() {
        if p.at(Name) {
            param(p)?;
        } else {
            break;
        }
    }
    p.expect(RParen)?;

    p.close(m, ParamList);
    Ok(())
}

// Param = TypeExpr 'name' ','?
fn param(p: &mut Parser) -> ParseResult<()> {
    assert!(p.at(Name));
    let m = p.open();
    type_expr(p)?;

    p.expect(Name)?;
    // p.expect(Colon);
    if !p.at(RParen) {
        p.expect(Comma)?;
    }
    p.close(m, Param);
    Ok(())
}

// TypeExpr = 'name'
fn type_expr(p: &mut Parser) -> ParseResult<()> {
    let m = p.open();
    p.expect(Name)?;
    p.close(m, TypeExpr);
    Ok(())
}

// Block = '{' Stmt* '}'
//
// Stmt =
//   StmtLet
// | StmtReturn
// | StmtExpr
fn block(p: &mut Parser) -> ParseResult<()> {
    assert!(p.at(LCurly));
    let m = p.open();
    p.expect(LCurly)?;
    while !p.at(RCurly) && !p.eof() {
        match p.nth(0) {
            //         LetKeyword => stmt_let(p),
            ReturnKeyword => stmt_return(p)?,
            //         _ => stmt_expr(p),
            stmt => todo!("handle stmt={:?}", stmt),
        }
    }
    p.expect(RCurly)?;
    p.close(m, Block);
    Ok(())
}

// StmtReturn = 'return' Expr ';'
fn stmt_return(p: &mut Parser) -> ParseResult<()> {
    assert!(p.at(ReturnKeyword));
    let m = p.open();
    p.expect(ReturnKeyword)?;
    expr(p)?;
    p.expect(Semi)?;
    p.close(m, StmtReturn);
    Ok(())
}

fn expr(p: &mut Parser) -> ParseResult<()> {
    expr_rec(p, Eof)?;
    Ok(())
}

fn expr_rec(p: &mut Parser, left: TokenKind) -> ParseResult<()> {
    let Some(mut lhs) = expr_delimited(p)? else {
        return Ok(());
    };

    while p.at(LParen) {
        // let m = p.open_before(lhs);
        todo!("expr call")
    }

    loop {
        let right = p.nth(0);
        if right_binds_tighter(left, right) {
            let m = p.open_before(lhs);
            p.advance();
            expr_rec(p, right)?;
            lhs = p.close(m, ExprBinary);
        } else {
            break;
        }
    }
    Ok(())
}

fn right_binds_tighter(left: TokenKind, right: TokenKind) -> bool {
    fn tightness(kind: TokenKind) -> Option<usize> {
        [
            // Precedence table:
            [Plus, Minus].as_slice(),
            &[Star, Slash],
        ]
        .iter()
        .position(|level| level.contains(&kind))
    }
    let Some(right_tightness) = tightness(right) else {
    return false
  };
    let Some(left_tightness) = tightness(left) else {
    assert!(left == Eof);
    return true;
  };
    right_tightness > left_tightness
}

fn expr_delimited(p: &mut Parser) -> ParseResult<Option<MarkClosed>> {
    let result = match p.nth(0) {
        TrueKeyword | FalseKeyword /*| Int*/ => {
            let m = p.open();
            p.advance();
            p.close(m, ExprLiteral)
        }
        Name => {
            let m = p.open();
            p.advance();
            p.close(m, ExprName)
        }
        LParen => {
            let m = p.open();
            p.expect(LParen)?;
            expr(p)?;
            p.expect(RParen)?;
            p.close(m, ExprParen)
        }
        _ => return Ok(None),
    };
    Ok(Some(result))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
