use crate::parser::Parser;
use crate::TokenKind::*;
use crate::TreeKind::*;

pub fn file(p: &mut Parser) {
    println!("rules.start");
    let m = p.open();
    dbg!(p.eof());
    while !p.eof() {
        dbg!(p.nth(0));
        if p.at(ClassKeyword) {
            class(p);
        } else {
            p.advance_with_error("expected an object");
        }
    }
    p.close(m, File);
}

fn class(p: &mut Parser) {
    assert!(p.at(ClassKeyword));
    let m = p.open();
    p.expect(ClassKeyword);
    p.expect(Name);
    p.expect(LCurly);
    while !p.at(RCurly) && !p.eof() {
        if method(p) {
            println!("found method");
        } else {
            p.advance_with_error("expected a method");
        }
    }
    p.expect(RCurly);
    p.close(m, Class);
}

// Fn = 'name' 'name' ParamList ('->' TypeExpr)? Block
fn method(p: &mut Parser) -> bool {
    // assert!(p.at(ClassKeyword));
    if p.nth(0) == Name && p.nth(1) == Name && p.nth(2) == LParen {
        let m = p.open();
        type_expr(p);
        p.expect(Name);

        if p.at(LParen) {
            param_list(p);
        }

        if p.at(LCurly) {
            block(p);
        }
        p.close(m, Method);
        return true;
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
    false
}

// ParamList = '(' Param* ')'
fn param_list(p: &mut Parser) {
    assert!(p.at(LParen));
    let m = p.open();

    p.expect(LParen);
    while !p.at(RParen) && !p.eof() {
        if p.at(Name) {
            param(p);
        } else {
            break;
        }
    }
    p.expect(RParen);

    p.close(m, ParamList);
}

// Param = TypeExpr 'name' ','?
fn param(p: &mut Parser) {
    assert!(p.at(Name));
    let m = p.open();
    type_expr(p);

    p.expect(Name);
    // p.expect(Colon);
    if !p.at(RParen) {
        p.expect(Comma);
    }
    p.close(m, Param);
}

// TypeExpr = 'name'
fn type_expr(p: &mut Parser) {
    let m = p.open();
    p.expect(Name);
    p.close(m, TypeExpr);
}

// Block = '{' Stmt* '}'
//
// Stmt =
//   StmtLet
// | StmtReturn
// | StmtExpr
fn block(p: &mut Parser) {
    assert!(p.at(LCurly));
    let m = p.open();
    p.expect(LCurly);
    while !p.at(RCurly) && !p.eof() {
        match p.nth(0) {
            //         LetKeyword => stmt_let(p),
            ReturnKeyword => stmt_return(p),
            //         _ => stmt_expr(p),
            stmt => todo!("handle stmt={:?}", stmt),
        }
    }
    p.expect(RCurly);
    p.close(m, Block);
}

// StmtReturn = 'return' Expr ';'
fn stmt_return(p: &mut Parser) {
    assert!(p.at(ReturnKeyword));
    let m = p.open();
    p.expect(ReturnKeyword);
    expr(p);
    p.expect(Semi);
    p.close(m, StmtReturn);
}

fn expr(p: &mut Parser) {
    expr_delimited(p)
}
fn expr_delimited(p: &mut Parser) {
    let m = p.open();
    match p.nth(0) {
        // ExprLiteral = 'int' | 'true' | 'false'
        // Int | TrueKeyword | FalseKeyword => {
        //     p.advance();
        //     p.close(m, ExprLiteral)
        // }
        // // ExprName = 'name'
        // Name => {
        //     p.advance();
        //     p.close(m, ExprName)
        // }
        // // ExprParen   = '(' Expr ')'
        // LParen => {
        //     p.expect(LParen);
        //     expr(p);
        //     p.expect(RParen);
        //     p.close(m, ExprParen)
        // }
        _ => {
            if !p.eof() {
                p.advance();
            }
            p.close(m, ErrorTree)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
