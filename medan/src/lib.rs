#![feature(exclusive_range_pattern)]

mod lexer;
mod syntax;

use lexer::parser;
use lexer::tokenizer;

pub fn run() {
    // let tokens = lexer::lex("a := 1; b := 2");
    // print!("{:?}", tokens);
    let mut parser = parser::Parser::new(tokenizer::lex(
        "secs := 22984415;
hours := secs / (60 * 60);
secs := secs - 60 * 60 * hours;

mins := secs / 60;
secs := secs - 60 * mins",
    ));
    // let res = parser.parse_aexpr();
    println!("{:#?}", parser.parse())
}
