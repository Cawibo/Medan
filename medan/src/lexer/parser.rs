use super::syntax::{parser::*, tokenizer::*};

pub struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let current_index = 0;
        Parser {
            tokens,
            current_index,
        }
    }

    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.current_index).map(|t| &t.kind)
    }

    fn eat(&mut self) -> () {
        self.current_index += 1
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current_index);
        if token.is_some() {
            self.current_index += 1
        }
        token
    }

    fn parse_literal(&mut self) -> Result<Expr, ErrorKind> {
        use TokenKind::*;
        match self.peek() {
            Some(IntLit(_)) | Some(BoolLit(_)) | Some(Identifier(_)) => (),
            Some(_) => return Err(ErrorKind::UnexpectedToken),
            None => return Err(ErrorKind::UnexpectedEOF),
        };

        let next = self.next().expect("unreachable");
        let lit_kind = match next.kind {
            IntLit(val) => ExprKind::IntLit(val),
            BoolLit(val) => ExprKind::BoolLit(val),
            Identifier(ref name) => ExprKind::Identifier(name.to_string()),
            ref other => panic!("Unreachable token kind: {:?}", other),
        };

        Ok(Expr { kind: lit_kind })
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ErrorKind> {
        use TokenKind::*;
        match self.peek() {
            Some(IntLit(_)) | Some(Identifier(_)) | Some(BoolLit(_)) => (),
            Some(LParen) => {
                self.eat(); // lparen
                let expr = self.parse_expr()?;
                self.eat(); // rparen
                return Ok(Expr {
                    kind: ExprKind::Precedence(Box::new(expr)),
                });
            }
            Some(Not) => {
                self.eat();
                let expr = self.parse_expr()?;
                return Ok(Expr {
                    kind: ExprKind::Negation(Box::new(expr)),
                });
            }
            Some(_) => return Err(ErrorKind::UnexpectedToken),
            None => return Err(ErrorKind::UnexpectedEOF),
        };

        // must be a literal unless bang.
        let left = self.parse_literal().expect("unreachable");

        match self.peek() {
            Some(Plus) | Some(Minus) | Some(Times) | Some(Slash) | Some(Equals) | Some(LEq)
            | Some(And) => (),
            None | Some(_) => return Ok(left),
        };

        let op_kind = if let Some(op) = self.next() {
            op.kind.clone()
        } else {
            unreachable!();
        };

        let right = self.parse_expr()?;

        Ok(Expr {
            kind: match op_kind {
                Plus => ExprKind::Add(Box::new(left), Box::new(right)),
                Minus => ExprKind::Sub(Box::new(left), Box::new(right)),
                Times => ExprKind::Mult(Box::new(left), Box::new(right)),
                Slash => ExprKind::Div(Box::new(left), Box::new(right)),
                And => ExprKind::Conjunction(Box::new(left), Box::new(right)),
                Equals => ExprKind::Equals(Box::new(left), Box::new(right)),
                LEq => ExprKind::LessThanOrEquals(Box::new(left), Box::new(right)),
                _ => panic!("what"),
            },
        })
    }

    pub fn parse(&mut self) -> Result<Stm, ErrorKind> {
        use TokenKind::*;
        let left = self.parse_statement()?;
        match self.peek() {
            Some(Semicolon) => {
                self.eat(); // Semicolon
                let right = self.parse()?;
                Ok(Stm {
                    kind: StmKind::Compound(Box::new(left), Box::new(right)),
                })
            }
            Some(_) | None => Ok(left),
        }
    }

    pub fn parse_statement(&mut self) -> Result<Stm, ErrorKind> {
        use TokenKind::*;
        match self.peek() {
            Some(Identifier(_)) | Some(If) | Some(While) => (),
            Some(_) => return Err(ErrorKind::UnexpectedToken),
            None => return Err(ErrorKind::UnexpectedEOF),
        };

        match self.peek() {
            Some(Identifier(name)) => {
                let identifier = name.to_string();
                self.eat();
                self.eat(); // assignment operator
                let expr = self.parse_expr()?;
                Ok(Stm {
                    kind: StmKind::Assignment(identifier, expr),
                })
            }
            Some(If) => {
                self.next(); // If
                let cond = self.parse_expr()?;
                self.next(); // Then
                let thn = self.parse_statement()?;
                self.next(); // Else
                let els = self.parse_statement()?;
                Ok(Stm {
                    kind: StmKind::Conditional(cond, Box::new(thn), Box::new(els)),
                })
            }
            Some(While) => {
                self.next(); // While
                let cond = self.parse_expr()?;
                self.next(); // Do
                let stm = self.parse_statement()?;
                Ok(Stm {
                    kind: StmKind::Loop(cond, Box::new(stm)),
                })
            }
            Some(_) => unreachable!(),
            None => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexer::lex;

    #[test]
    fn test_peek() {
        let parser = Parser::new(lex("a := 1"));
        assert_eq!(
            parser.peek(),
            Some(&TokenKind::Identifier(String::from("a")))
        );
    }

    #[test]
    fn test_next() {
        let mut parser = Parser::new(lex("a := 1"));
        assert_eq!(parser.current_index, 0);
        parser.next();
        assert_eq!(parser.current_index, 1);
    }
}
