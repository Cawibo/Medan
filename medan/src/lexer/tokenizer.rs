use crate::syntax::lexer::*;

fn lex_alphas(it: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
    let mut alphas: String = String::new();

    while let Some(&c) = it.peek() {
        match c {
            'a'..='z' => {
                it.next();
                alphas.push(c);
            }
            _ => break,
        }
    }

    alphas
}

fn lex_number(it: &mut std::iter::Peekable<std::str::Chars<'_>>) -> usize {
    let mut number: String = String::new();

    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                number.push(c);
            }
            _ => break,
        }
    }

    number.parse::<usize>().unwrap()
}

fn lex_whitespace(
    it: &mut std::iter::Peekable<std::str::Chars<'_>>,
    position: &mut Position,
) -> TokenKind {
    while let Some(&c) = it.peek() {
        match c {
            '\n' => {
                position.line += 1;
                position.offset = 1;
            }
            '\t' => position.offset += 4,
            ' ' => position.offset += 1,
            _ => break,
        };
        it.next();
    }

    TokenKind::Whitespace
}

fn lex_meta(
    it: &mut std::iter::Peekable<std::str::Chars<'_>>,
    position: &mut Position,
) -> TokenKind {
    let kind = match it.peek().unwrap() {
        '+' => TokenKind::Plus,
        '-' => TokenKind::Minus,
        '*' => TokenKind::Times,
        '!' => TokenKind::Bang,
        '(' => TokenKind::LParen,
        '&' => TokenKind::And,
        ')' => TokenKind::RParen,
        '/' => TokenKind::Division,
        ';' => TokenKind::Semicolon,
        ':' => {
            it.next();
            position.offset += 1;
            match it.peek() {
                Some('=') => TokenKind::Assign,
                Some(c) => panic!("unexpected character {} at {:?}", c, position),
                None => panic!("unexpected EOF"),
            }
        }
        '<' => {
            it.next();
            position.offset += 1;
            match it.peek() {
                Some('=') => TokenKind::LEq,
                Some(c) => panic!("unexpected character {} at {:?}", c, position),
                None => panic!("unexpected EOF"),
            }
        }
        '=' => TokenKind::Equals,
        c => panic!("unexpected character {} at {:?}", c, position),
    };

    it.next();

    kind
}

pub fn lex(input: &'static str) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut position = Position { line: 1, offset: 1 };

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        let kind = match c {
            'a'..'z' => {
                let alphas = lex_alphas(&mut it);
                position.offset += alphas.len();
                match alphas.as_str() {
                    "skip" => TokenKind::Skip,
                    "true" => TokenKind::BoolLit(true),
                    "false" => TokenKind::BoolLit(false),
                    "while" => TokenKind::While,
                    "then" => TokenKind::Then,
                    "else" => TokenKind::Else,
                    "do" => TokenKind::Do,
                    "if" => TokenKind::If,
                    _ => TokenKind::Identifier(alphas),
                }
            }
            '0'..'9' => {
                let n = lex_number(&mut it);
                position.offset += format!("{}", n).len();
                TokenKind::IntLit(n)
            }
            '\n' | '\t' | ' ' => lex_whitespace(&mut it, &mut position),
            _ => lex_meta(&mut it, &mut position),
        };
        if kind == TokenKind::Whitespace {
            continue;
        }
        result.push(Token {
            kind: kind,
            position: position.clone(),
        })
    }

    result
}
