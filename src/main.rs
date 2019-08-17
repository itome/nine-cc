mod node;
mod token;
mod generator;

use node::Node;
use std::env;
use token::Token;
use generator::generate_program;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg: &String = &args[1];
    let mut tokens = Token::parse(arg.to_string());
    let program = Node::program(&mut tokens);
    let assembly = generate_program(&program);
    for line in assembly {
        println!("{}", line);
    }
}
