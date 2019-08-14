mod node;
mod token;

use node::Node;
use std::env;
use token::Token;

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
        Some(op) => match op.as_ref() {
            "+" => {
                println!("  add rax, rdi");
            }
            "-" => {
                println!("  sub rax, rdi");
            }
            "*" => {
                println!("  imul rax, rdi");
            }
            "/" => {
                println!("  cqo");
                println!("  idiv rdi");
            }
            "==" => {
                println!("  cmp rax, rdi");
                println!("  sete al");
                println!("  movzb rax, al",);
            }
            "!=" => {
                println!("  cmp rax, rdi");
                println!("  setne al");
                println!("  movzb rax, al",);
            }
            "<" => {
                println!("  cmp rax, rdi");
                println!("  setl al");
                println!("  movzb rax, al",);
            }
            "<=" => {
                println!("  cmp rax, rdi");
                println!("  setle al");
                println!("  movzb rax, al",);
            }
            _ => {}
        },
        _ => {}
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
