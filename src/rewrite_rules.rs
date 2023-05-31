use crate::{tree::Child, TokenKind, Tree, TreeKind};
use std::fmt::{self, Write};

pub fn file(tree: &Tree, dst: &mut String) -> fmt::Result {
    assert!(tree.kind == TreeKind::File);
    for child in &tree.children {
        match child {
            Child::Tree(child) if child.kind == TreeKind::Class => class(child, dst)?,
            _ => todo!("handle error"),
        }
    }
    Ok(())
}

fn class(tree: &Tree, dst: &mut String) -> fmt::Result {
    assert!(tree.kind == TreeKind::Class);
    // dbg!(tree);
    class_decl(tree, dst)?;
    class_impl(tree, dst)?;
    Ok(())
}

fn class_decl(tree: &Tree, dst: &mut String) -> fmt::Result {
    writeln!(
        dst,
        "struct {} {{",
        tree.child_value_by_token(TokenKind::Name).expect("name")
    )?;
    writeln!(dst, "}}")
}

fn class_impl(tree: &Tree, dst: &mut String) -> fmt::Result {
    writeln!(
        dst,
        "impl {} {{",
        tree.child_value_by_token(TokenKind::Name).unwrap_or("")
    )?;
    methods(tree, dst)?;
    // for child in &tree.children {
    //     match child {
    //         Child::Token(token) => {
    //             println!("token={:?}", token);
    //             match token.kind {
    //                 TokenKind::ClassKeyword => {}
    //                 TokenKind::Name => {
    //                     dst.push_str(&format!("struct {}", token.text));
    //                 }
    //                 TokenKind::LCurly => {
    //                     dst.push_str("{\n");
    //                 }
    //                 TokenKind::RCurly => {
    //                     dst.push_str("}\n");
    //                 }
    //                 _ => todo!("handle error"),
    //             }
    //         }
    //         Child::Tree(method_tree) if method_tree.kind == TreeKind::Method => {
    //             method(method_tree, dst)
    //         }
    //         x => todo!("handle unexpected {:?}", x),
    //     }
    // }
    writeln!(dst, "}}")
}

fn methods(tree: &Tree, dst: &mut String) -> fmt::Result {
    for method_tree in tree.children_by_tree(TreeKind::Method) {
        method(method_tree, dst)?;
    }
    Ok(())
}

fn method(tree: &Tree, dst: &mut String) -> fmt::Result {
    assert!(tree.kind == TreeKind::Method);
    // dbg!(tree);
    write!(
        dst,
        "    fn {}",
        tree.child_value_by_token(TokenKind::Name)
            .expect("method name")
    )?;
    if let Some(p) = tree.child_by_tree(TreeKind::ParamList) {
        param_list(p, dst)?;
    } else {
        todo!("handle error")
    }
    if let Some(te) = tree.child_by_tree(TreeKind::TypeExpr) {
        write!(dst, " -> ")?;
        type_expr(te, dst)?;
    } else {
        todo!("handle no type expr");
    }
    if let Some(b) = tree.child_by_tree(TreeKind::Block) {
        block(b, dst)?;
    } else {
        todo!("handle error")
    }
    Ok(())
}
fn param_list(tree: &Tree, dst: &mut String) -> fmt::Result {
    assert!(tree.kind == TreeKind::ParamList);
    // dbg!(tree);
    write!(dst, "(",)?;
    for (i, p) in tree.children_by_tree(TreeKind::Param).enumerate() {
        if i > 0 {
            write!(dst, ", ")?;
        }
        param(p, dst)?;
    }
    // if let Some(p) = tree.child_by_tree(TreeKind::ParamList) {
    //     param_list(p, dst)?;
    // } else {
    //     todo!("handle error")
    // }
    // for child in &tree.children {
    //     match child {
    //         Child::Token(token) => {
    //             println!("token={:?}", token);
    //             match token.kind {
    //                 TokenKind::ClassKeyword => {}
    //                 TokenKind::Name => {
    //                     dst.push_str(&format!("struct {}", token.text));
    //                 }
    //                 TokenKind::LCurly => {
    //                     dst.push_str("{\n");
    //                 }
    //                 TokenKind::RCurly => {
    //                     dst.push_str("}\n");
    //                 }
    //                 _ => todo!("handle error"),
    //             }
    //         }
    //         // Child::Tree()
    //         x => todo!("handle unexpected {:?}", x),
    //     }
    // }
    write!(dst, ")",)?;
    Ok(())
}
fn param(tree: &Tree, dst: &mut String) -> fmt::Result {
    assert!(tree.kind == TreeKind::Param);
    // dbg!(tree);

    write!(
        dst,
        "{}: ",
        tree.child_value_by_token(TokenKind::Name)
            .expect("param name")
    )?;

    if let Some(te) = tree.child_by_tree(TreeKind::TypeExpr) {
        type_expr(te, dst)?;
    } else {
        todo!("handle no type expr");
    }

    Ok(())
}
fn type_expr(tree: &Tree, dst: &mut String) -> fmt::Result {
    assert!(tree.kind == TreeKind::TypeExpr);
    // dbg!(tree);

    write!(
        dst,
        "{}",
        tree.child_value_by_token(TokenKind::Name)
            .expect("param name")
    )?;

    Ok(())
}

fn block(tree: &Tree, dst: &mut String) -> fmt::Result {
    assert!(tree.kind == TreeKind::Block);
    // dbg!(tree);

    writeln!(dst, " {{",)?;
    for stmt_tree in tree.children_by_tree(TreeKind::StmtReturn) {
        match stmt_tree.kind {
            TreeKind::StmtReturn => stmt_return(stmt_tree, dst)?,
            _ => todo!("handle stmt"),
        }
    }
    writeln!(dst, "    }}",)?;

    Ok(())
}

fn stmt_return(tree: &Tree, dst: &mut String) -> fmt::Result {
    assert!(tree.kind == TreeKind::StmtReturn);
    // dbg!(tree);
    write!(dst, "return ",)?;
    expr(
        tree.child_by_tree(TreeKind::ExprBinary).expect("child"),
        dst,
    )?;
    writeln!(dst, ";",)?;

    Ok(())
}

fn expr(tree: &Tree, dst: &mut String) -> fmt::Result {
    // assert!(tree.kind == TreeKind::Block);
    // dbg!(tree);
    match tree.kind {
        TreeKind::ExprBinary => {
            for child in &tree.children {
                match child {
                    Child::Tree(e) => write!(
                        dst,
                        "{}",
                        e.child_value_by_token(TokenKind::Name).expect("expr name")
                    )?,
                    Child::Token(token) => write!(dst, " {} ", token.text)?,
                }
            }
        }
        _ => todo!("handle expr"),
    }
    Ok(())
}
