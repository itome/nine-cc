use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg: i32 = args[1].parse().unwrap();
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", arg);
    println!("  ret")
}
