use super::syntax::compiler::*;
use crate::lexer::syntax::parser::*;

fn compile_expr(ast: Expr) -> Vec<Code> {
    let mut res: Vec<Code> = Vec::new();

    use ExprKind::*;
    match ast.kind {
        IntLit(val) => res.push(Code::PUSH(val)),
        _ => panic!("unimplemented"),
    };

    res
}

fn compile_statement(ast: Stm) -> Vec<Code> {
    let mut res: Vec<Code> = Vec::new();
    use StmKind::*;
    match ast.kind {
        Assignment(name, expr) => {
            res.extend(compile_expr(expr));
            res.push(Code::STORE(name));
        }
        _ => panic!("unimplemented"),
    };

    res
}

pub fn compile(ast: Stm) -> Vec<Code> {
    let mut res: Vec<Code> = Vec::new();

    res.extend(compile_statement(ast));

    res
}
