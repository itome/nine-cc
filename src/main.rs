use nine_cc::strtol;
use std::env;
use std::process::exit;

#[derive(Debug, Clone)]
struct Token {
    value: Option<i64>,
    operator: Option<char>,
}

#[derive(Debug)]
struct Node {
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
    number: Option<i64>,
    operator: Option<char>,
}

impl Node {
    fn operator(op: char, lhs: Node, rhs: Node) -> Self {
        Node {
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
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

    fn expr(tokens: &mut Vec<Token>) -> Self {
        let mut node = Node::mul(tokens);

        loop {
            if tokens.len() == 0 {
                break;
            }
            let token = &tokens[0];
            match token.operator {
                Some('+') => {
                    tokens.remove(0);
                    let rhs = Node::mul(tokens);
                    node = Node::operator('+', node, rhs);
                }
                Some('-') => {
                    tokens.remove(0);
                    let rhs = Node::mul(tokens);
                    node = Node::operator('-', node, rhs);
                }
                _ => {
                    break;
                }
            }
        }
        return node;
    }

    fn mul(tokens: &mut Vec<Token>) -> Self {
        let mut node = Node::term(tokens);

        loop {
            if tokens.len() == 0 {
                break;
            }
            let token = &tokens[0];
            match token.operator {
                Some('*') => {
                    tokens.remove(0);
                    let rhs = Node::term(tokens);
                    node = Node::operator('*', node, rhs);
                }
                Some('/') => {
                    tokens.remove(0);
                    let rhs = Node::term(tokens);
                    node = Node::operator('/', node, rhs);
                }
                _ => {
                    break;
                }
            }
        }
        return node;
    }

    fn term(tokens: &mut Vec<Token>) -> Self {
        if tokens[0].operator == Some('(') {
            let close_index = tokens
                .iter()
                .position(|token| token.operator == Some(')'))
                .unwrap();
            let mut exp = tokens[1..(close_index - 1)].to_vec();
            tokens.drain(0..close_index);
            return Node::expr(&mut exp);
        } else {
            let num = tokens[0].value.unwrap();
            tokens.remove(0);
            return Node::number(num);
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

        if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' {
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
    let mut tokens = tokenize(arg.to_string());
    let expr = Node::expr(&mut tokens);
    println!("{:?}", expr);
}
