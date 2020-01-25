use super::syntax::tokenizer::*;

pub struct Tokenizer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
    current_index: usize,
}

impl Tokenizer<'static> {
    pub fn new(string: &'static str) -> Tokenizer<'static> {
        Tokenizer {
            input: string.chars().peekable(),
            current_index: 0,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn next(&mut self) -> Option<char> {
        self.input.next()
    }

    fn lex_numeral(&mut self) -> Result<usize, TokenizerError> {
        let mut res: String = String::new();

        while let Some(&c) = self.input.peek() {
            match c {
                '0'..='9' => res.push(c),
                _ => break,
            }
            self.input.next();
        }

        let res_val = res.parse::<usize>().expect("unreachable");

        Ok(res_val)
    }

    fn lex_alpha(&mut self) -> Result<String, TokenizerError> {
        let mut res: String = String::new();

        while let Some(&c) = self.input.peek() {
            match c {
                'a'..='z' => res.push(c),
                _ => break,
            }
            self.input.next();
        }

        Ok(res)
    }

    // meta characters include: +-*/()&;:=<
    fn lex_meta(&mut self) -> Result<String, TokenizerError> {
        let mut res: String = String::new();
        let special: &str = ":";

        let c = self.input.next().expect("unreachable");
        res.push(c);

        if let Some(&d) = self.input.peek() {
            if special.contains(c) {
                res.push(d);
                self.input.next();
            }
        }

        Ok(res)
    }

    pub fn run(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut res: Vec<Token> = Vec::new();

        while let Some(&c) = self.input.peek() {
            let kind = match c {
                '0'..='9' => {
                    let number = self.lex_numeral()?;
                    TokenKind::IntLit(number)
                }
                'a'..='z' => {
                    let alpha = self.lex_alpha()?;
                    match alpha.as_str() {
                        "while" => TokenKind::While,
                        "if" => TokenKind::If,
                        "and" => TokenKind::And,
                        "not" => TokenKind::Not,
                        "do" => TokenKind::Do,
                        "else" => TokenKind::Else,
                        "then" => TokenKind::Then,
                        "skip" => TokenKind::Skip,
                        "true" => TokenKind::BoolLit(true),
                        "false" => TokenKind::BoolLit(false),
                        ident => TokenKind::Identifier(ident.to_string()),
                    }
                }
                c if c.is_whitespace() => {
                    self.input.next();
                    continue;
                }
                _ => {
                    let meta = self.lex_meta()?;
                    match meta.as_str() {
                        "+" => TokenKind::Plus,
                        "-" => TokenKind::Minus,
                        "*" => TokenKind::Times,
                        "/" => TokenKind::Slash,
                        "(" => TokenKind::LParen,
                        ")" => TokenKind::RParen,
                        ":=" => TokenKind::Assign,
                        "<=" => TokenKind::LEq,
                        ";" => TokenKind::Semicolon,
                        err => return Err(TokenizerError::UnexpectedMeta(err.to_string())),
                    }
                }
            };

            res.push(Token { kind: kind })
        }

        Ok(res)
    }
}
