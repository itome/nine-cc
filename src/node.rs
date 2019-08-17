use crate::token::Token;

#[derive(Debug)]
pub struct Node {
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub number: Option<i64>,
    pub operator: Option<String>,
    pub offset: Option<usize>,
}

pub struct Parser {
    current_offset: usize
}

impl Parser {
    pub fn new() -> Self {
        return Parser { current_offset: 0 };
    }

    fn operator(op: String, lhs: Node, rhs: Node) -> Node {
        Node {
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
            number: None,
            operator: Some(op),
            offset: None,
        }
    }

    fn number(num: i64) -> Node {
        Node {
            lhs: None,
            rhs: None,
            number: Some(num),
            operator: None,
            offset: None,
        }
    }

    fn ident(ident: String) -> Node {
        let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let index = alphabet.iter().position(|&c| c == ident.chars().next().unwrap()).unwrap();
        Node {
            lhs: None,
            rhs: None,
            number: None,
            operator: None,
            offset: Some((index + 1) * 8),
        }
    }

    pub fn program(self: &Self, tokens: &mut Vec<Token>) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec!();
        while !tokens.is_empty() {
            nodes.push(self.stmt(tokens));
        };
        return nodes;
    }

    fn stmt(self: &Self, tokens: &mut Vec<Token>) -> Node {
        let node = self.expr(tokens);
        tokens.remove(0);
        return node;
    }

    fn expr(self: &Self, tokens: &mut Vec<Token>) -> Node {
        return self.assign(tokens);
    }

    fn assign(self: &Self, tokens: &mut Vec<Token>) -> Node {
        let mut node = self.equality(tokens);
        match &tokens.first() {
            Some(token) if token.operator == Some("=".to_string()) => {
                tokens.remove(0);
                let rhs = self.assign(tokens);
                node = Parser::operator("=".to_string(), node, rhs)
            }
            _ => {}
        }
        return node;
    }

    fn equality(self: &Self, tokens: &mut Vec<Token>) -> Node {
        let mut node = self.relational(tokens);

        loop {
            if tokens.len() == 0 {
                break;
            }
            let token = &tokens[0];
            match &token.operator {
                Some(op) => match op.as_ref() {
                    "==" => {
                        tokens.remove(0);
                        let rhs = self.relational(tokens);
                        node = Parser::operator("==".to_string(), node, rhs);
                    }
                    "!=" => {
                        tokens.remove(0);
                        let rhs = self.relational(tokens);
                        node = Parser::operator("!=".to_string(), node, rhs);
                    }
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }
        return node;
    }

    fn relational(self: &Self, tokens: &mut Vec<Token>) -> Node {
        let mut node = self.add(tokens);

        loop {
            if tokens.len() == 0 {
                break;
            }
            let token = &tokens[0];
            match &token.operator {
                Some(op) => match op.as_ref() {
                    "<" => {
                        tokens.remove(0);
                        let rhs = self.mul(tokens);
                        node = Parser::operator("<".to_string(), node, rhs);
                    }
                    "<=" => {
                        tokens.remove(0);
                        let rhs = self.mul(tokens);
                        node = Parser::operator("<=".to_string(), node, rhs);
                    }
                    ">" => {
                        tokens.remove(0);
                        let rhs = self.mul(tokens);
                        node = Parser::operator("<".to_string(), rhs, node);
                    }
                    ">=" => {
                        tokens.remove(0);
                        let rhs = self.mul(tokens);
                        node = Parser::operator("<=".to_string(), rhs, node);
                    }
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }
        return node;
    }

    fn add(self: &Self, tokens: &mut Vec<Token>) -> Node {
        let mut node = self.mul(tokens);

        loop {
            if tokens.len() == 0 {
                break;
            }
            let token = &tokens[0];
            match &token.operator {
                Some(op) => match op.as_ref() {
                    "+" => {
                        tokens.remove(0);
                        let rhs = self.mul(tokens);
                        node = Parser::operator("+".to_string(), node, rhs);
                    }
                    "-" => {
                        tokens.remove(0);
                        let rhs = self.mul(tokens);
                        node = Parser::operator("-".to_string(), node, rhs);
                    }
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }
        return node;
    }

    fn mul(self: &Self, tokens: &mut Vec<Token>) -> Node {
        let mut node = self.unary(tokens);

        loop {
            if tokens.len() == 0 {
                break;
            }
            let token = &tokens[0];
            match &token.operator {
                Some(op) => match op.as_ref() {
                    "*" => {
                        tokens.remove(0);
                        let rhs = self.unary(tokens);
                        node = Parser::operator("*".to_string(), node, rhs);
                    }
                    "/" => {
                        tokens.remove(0);
                        let rhs = self.unary(tokens);
                        node = Parser::operator("/".to_string(), node, rhs);
                    }
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }
        return node;
    }

    fn unary(self: &Self, tokens: &mut Vec<Token>) -> Node {
        let token = &tokens[0];
        match &token.operator {
            Some(op) => match op.as_ref() {
                "+" => {
                    tokens.remove(0);
                    return self.term(tokens);
                }
                "-" => {
                    tokens.remove(0);
                    return Parser::operator("-".to_string(), Parser::number(0), self.term(tokens));
                }
                _ => {
                    return self.term(tokens);
                }
            },
            _ => {
                return self.term(tokens);
            }
        }
    }

    fn term(self: &Self, tokens: &mut Vec<Token>) -> Node {
        match &tokens[0].operator {
            Some(op) if op == "(" => {
                let close_index = tokens
                    .iter()
                    .position(|token| token.operator == Some(")".to_string()))
                    .unwrap();
                let mut exp = tokens[1..close_index].to_vec();
                tokens.drain(0..(close_index + 1));
                return self.expr(&mut exp);
            }
            _ => match &tokens[0].ident {
                Some(ident) => {
                    let ident = ident.clone();
                    tokens.remove(0);
                    return Parser::ident(ident);
                }
                _ => {
                    let num = tokens[0].number.unwrap();
                    tokens.remove(0);
                    return Parser::number(num);
                }
            },
        }
    }
}
