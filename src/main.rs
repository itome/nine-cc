use nine_cc::strtol;
use std::env;
use std::process::exit;

#[derive(Debug)]
enum TokenKind {
    Operator,
    Number,
    Eof,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    value: Option<i64>,
    operator: Option<char>,
}

fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut input = input.clone();
    while let Some(c) = input.chars().nth(0) {
        if c.is_whitespace() {
            input = input.split_off(1);
            continue;
        }

        if c == '+' || c == '-' {
            let token = Token {
                kind: TokenKind::Operator,
                value: None,
                operator: Some(c),
            };
            input = input.split_off(1);
            tokens.push(token);
            continue;
        }

        if c.is_ascii_digit() {
            let (num, remaining) = strtol(&input);
            input = remaining;
            let token = Token {
                kind: TokenKind::Number,
                value: num,
                operator: None,
            };
            tokens.push(token);
            continue;
        }

        eprintln!("cannot tokenize: {}", c);
        exit(1);
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        value: None,
        operator: None,
    });

    return tokens;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg: &String = &args[1];
    let tokens = tokenize(arg.to_string());
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    for (index, token) in tokens.iter().enumerate() {
        if index == 0 {
            println!("  mov rax, {}", token.value.unwrap());
            continue;
        }
        match token.kind {
            TokenKind::Number => match tokens[index - 1].operator {
                Some('+') => {
                    println!("  add rax, {}", token.value.unwrap());
                }
                Some('-') => {
                    println!("  sub rax, {}", token.value.unwrap());
                }
                Some(_) | None => {
                    panic!("operator not found");
                }
            },
            TokenKind::Operator => {}
            TokenKind::Eof => {
                println!("  ret");
            }
        }
    }
}
