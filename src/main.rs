use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg: &str = &args[1];
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, 0");

    let mut operator: char = '+';
    let mut num: String = "".to_string();
    let arg = format!("{}{}", arg, "e");
    for char in arg.chars() {
        if char == '+' || char == '-' || char == 'e' {
            if operator == '+' {
                println!("  add rax, {}", num.parse::<usize>().unwrap());
            }
            if operator == '-' {
                println!("  sub rax, {}", num.parse::<usize>().unwrap());
            }
            operator = char;
            num = "".to_string();
        } else {
            num += &char.to_string();
        }
    }

    println!("  ret")
}
