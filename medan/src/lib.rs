#![feature(exclusive_range_pattern)]

pub mod ast;
pub mod lexer;
use ast::compiler;
use lexer::parser;
use lexer::tokenizer;

pub fn run() {
    let mut tokenizer = tokenizer::Tokenizer::new("a := 0");
    let res = tokenizer.run().expect("er");
    println!("res: {:?}", res);

    let mut parser = parser::Parser::new(res);
    let res = parser.parse().expect("er");
    println!("res: {:?}", res);

    let code = compiler::compile(res);
    println!("code: {:?}", code);

    // let tokens = lexer::lex("a := 1; b := 2");
    // print!("{:?}", tokens);
    //     let mut parser = parser::Parser::new(tokenizer::run(
    //         "secs := 22984415;
    // hours := secs / (60 * 60);
    // secs := secs - 60 * 60 * hours;

    // mins := secs / 60;
    // secs := secs - 60 * mins",
    //     ));
    // let res = parser.parse_aexpr();
    // println!("{:#?}", parser.parse())
}
