use crate::tispc_lexer::{Ident, LiteralKind, Token, TokenKind, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    Constant(Value<'a>),
    Builtin(Ident<'a>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
}

/// generate_expression_tree
///
/// Takes in a stream of `Token`s and generates an Expression
/// tree of type Vec<Expr>

pub fn generate_expression_tree(token_stream: Vec<Token>) -> Vec<Expr> {
    let mut stack: Vec<Expr> = Vec::new();
    for token in token_stream {
        let expr = match token.kind {
            TokenKind::OpenParen => continue,
            // TODO: Create a Operator TokenKind for the following group
            TokenKind::Plus => Some(Expr::Builtin(Ident::Plus)),
            TokenKind::Minus => Some(Expr::Builtin(Ident::Minus)),
            TokenKind::Mult => Some(Expr::Builtin(Ident::Mult)),
            TokenKind::Divide => Some(Expr::Builtin(Ident::Div)),
            TokenKind::Ident => {
                // TODO: Add logic to further divide Ident into categories (later)
                match token.value {
                    Some(Value::String(val)) => Some(Expr::Builtin(Ident::FuncName(val))),
                    _ => panic!("Invalid value for Identifier"),
                }
            }
            TokenKind::Literal(LiteralKind::Boolean) => Some(Expr::Constant(token.value.unwrap())),
            TokenKind::Literal(LiteralKind::Number) => Some(Expr::Constant(token.value.unwrap())),
            TokenKind::Literal(LiteralKind::String) => match token.value {
                Some(Value::String(str)) => Some(Expr::Constant(Value::String(str))),
                _ => panic!("Invalid value for string literal"),
            },

            TokenKind::CloseParen => {
                let mut params: Vec<Expr> = Vec::new();
                // pop elements from stack until a Builtin is found
                loop {
                    let expr = stack.pop();
                    match expr {
                        Some(Expr::Builtin(_)) => {
                            params.push(expr.unwrap());
                            break;
                        }
                        _ => params.push(expr.unwrap()),
                    }
                }
                let func_name = params.pop().unwrap();

                // create Expr from params and func name
                Some(Expr::Call(Box::new(func_name), params))
            }

            _ => panic!("Invaid expression"),
        };

        stack.push(expr.unwrap());
    }

    stack
}
