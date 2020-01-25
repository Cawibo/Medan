pub mod parser {
    use super::lexer::*;
    use std::fmt;

    #[derive(Debug)]
    pub enum StmKind {
        Assignment(String, Expr),
        Conditional(Expr, Box<Stm>, Box<Stm>),
        Loop(Expr, Box<Stm>),
        Compound(Box<Stm>, Box<Stm>),
    }

    pub struct Stm {
        pub kind: StmKind,
        // pub position: Position,
    }

    impl fmt::Debug for Stm {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use StmKind::*;
            match &self.kind {
                Assignment(id, expr) => write!(f, "Assign( {:?}, {:?} )", id, expr),
                Conditional(cond, expr1, expr2) => {
                    write!(f, "Cond( {:?}, {:?}, {:?} )", cond, expr1, expr2)
                }
                Loop(cond, expr) => write!(f, "Loop( {:?}, {:?} )", cond, expr),
                Compound(left, right) => write!(f, "{:?}; {:?}", left, right),
            }
        }
    }

    pub enum ExprKind {
        IntLit(usize),
        BoolLit(bool),
        Identifier(String),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
        Mult(Box<Expr>, Box<Expr>),
        Div(Box<Expr>, Box<Expr>),
        Equals(Box<Expr>, Box<Expr>),
        LessThanOrEquals(Box<Expr>, Box<Expr>),
        Negation(Box<Expr>),
        Conjunction(Box<Expr>, Box<Expr>),
        Precedence(Box<Expr>),
    }

    pub struct Expr {
        pub kind: ExprKind,
        pub position: Position,
    }

    impl fmt::Debug for Expr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use ExprKind::*;
            match &self.kind {
                IntLit(val) => write!(f, "{:?}", val),
                BoolLit(val) => write!(f, "{:?}", val),
                Identifier(name) => write!(f, "{:?}", name),
                Negation(expr) => write!(f, "Not( {:?} )", expr),
                Add(left, right) => write!(f, "Add( {:?}, {:?} )", left, right),
                Sub(left, right) => write!(f, "Sub( {:?}, {:?} )", left, right),
                Mult(left, right) => write!(f, "Mult( {:?}, {:?} )", left, right),
                Div(left, right) => write!(f, "Div( {:?}, {:?} )", left, right),
                Equals(left, right) => write!(f, "Equals( {:?}, {:?} )", left, right),
                LessThanOrEquals(left, right) => {
                    write!(f, "LessThanOrEquals( {:?}, {:?} )", left, right)
                }
                Conjunction(left, right) => write!(f, "Conjunction( {:?}, {:?} )", left, right),
                Precedence(expr) => write!(f, "{:?}", expr),
            }
        }
    }

    #[derive(Debug)]
    pub enum ErrorKind {
        UnexpectedEOF,
        UnexpectedToken,
    }
}

pub mod lexer {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum TokenKind {
        LParen,
        RParen,
        Plus,
        Minus,
        Times,
        Division,
        Assign,
        Equals,
        Skip,
        LEq,
        While,
        Then,
        Else,
        Do,
        If,
        And,
        Semicolon,
        Bang,
        Whitespace,
        Identifier(String),
        IntLit(usize),
        BoolLit(bool),
    }

    #[derive(Clone, Copy)]
    pub struct Position {
        pub line: usize,
        pub offset: usize,
    }

    pub struct Token {
        pub kind: TokenKind,
        pub position: Position,
    }

    use std::fmt;

    impl fmt::Debug for Token {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.kind)
        }
    }

    impl fmt::Debug for Position {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}:{})", self.line, self.offset)
        }
    }
}
