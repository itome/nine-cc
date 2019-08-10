mod token;
mod node;

use node::Node;
use token::Token;
use std::env;

fn gen(node: &Node) {
    if let Some(num) = node.number {
        println!("  push {}", num);
        return;
    }

    if let Some(rhs) = &node.rhs {
        gen(&rhs);
    }
    if let Some(lhs) = &node.lhs {
        gen(&lhs);
    }
    println!("  pop rax");
    println!("  pop rdi");

    match &node.operator {
        Some('+') => {
            println!("  add rax, rdi");
        }
        Some('-') => {
            println!("  sub rax, rdi");
        }
        Some('*') => {
            println!("  imul rax, rdi");
        }
        Some('/') => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        _ => {
        }
    }
    println!("  push rax");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg: &String = &args[1];
    let mut tokens = Token::parse(arg.to_string());
    let expr = Node::expr(&mut tokens);
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    gen(&expr);
    println!("  pop rax");
    println!("  ret");
}
