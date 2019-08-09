use nine_cc::strtol;
use std::env;
use std::process::exit;

#[derive(Debug, Clone)]
struct Token {
    value: Option<i64>,
    operator: Option<char>,
}

struct Node {
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
    number: Option<i64>,
    operator: Option<char>,
}

impl Node {
    fn operator(op: char, lhs: Node, rhs: Node) -> Self {
        Node {
            lhs: Some(lhs),
            rhs: Some(rhs),
            number: None,
            operator: Some(op),
        }
    }

    fn number(num: i64) -> Self {
        Node {
            lhs: None,
            rhs: None,
            number: Some(num),
            operator: None,
        }
    }

    fn expr(mut tokens: Vec<Token>) -> (Self, Vec<Token>) {
        let (mut node, tokens) = Node::mul(tokens);

        for token in tokens {
            match token.operator {
                Some('+') => {
                    let (rhs, tokens) = Node::mul(tokens[1..].to_vec());
                    node = Node::operator('+', node, rhs);
                }
                Some('-') => {
                    let (rhs, tokens) = Node::mul(tokens[1..].to_vec());
                    node = Node::operator('-', node, rhs);
                }
            }
        }
        return (node, tokens);
    }

    fn mul(mut tokens: Vec<Token>) -> (Self, Vec<Token>) {
        let (mut node, tokens) = Node::term(tokens);

        for token in tokens {
            match token.operator {
                Some('*') => {
                    let (rhs, tokens) = Node::term(tokens[1..].to_vec());
                    node = Node::operator('*', node, rhs);
                }
                Some('/') => {
                    let (rhs, tokens) = Node::term(tokens[1..].to_vec());
                    node = Node::operator('/', node, rhs);
                }
            }
        }
        return (node, tokens);
    }

    fn term(tokens: Vec<Token>) -> (Self, Vec<Token>) {
        if tokens[0].operator == Some('(') {
            let close_index = tokens
                .iter()
                .position(|token| token.operator == Some(')'))
                .unwrap();
            return Node::expr(tokens[1..(close_index - 1)].to_vec());
        } else {
            return (Node::number(tokens[0].value.unwrap()), tokens[1..].to_vec());
        };
    }
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
        if let Some(value) = token.value {
            match tokens[index - 1].operator {
                Some('+') => {
                    println!("  add rax, {}", value);
                }
                Some('-') => {
                    println!("  sub rax, {}", value);
                }
                Some(_) | None => {
                    println!("operator not found");
                }
            }
        }
        if token.value == None && token.operator == None {
            println!("  ret");
        }
    }
}
