mod node;
mod token;
mod generator;

use node::Parser;
use std::env;
use token::Token;
use generator::gen_program;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg: &String = &args[1];
    let mut tokens = Token::parse(arg.to_string());
    let program = Parser::new().program(&mut tokens);
    let assembly = gen_program(&program);
    for line in assembly {
        println!("{}", line);
    }
}
