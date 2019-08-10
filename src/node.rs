use crate::token::Token;

#[derive(Debug)]
pub struct Node {
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub number: Option<i64>,
    pub operator: Option<char>,
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

    pub fn expr(tokens: &mut Vec<Token>) -> Self {
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
        let mut node = Node::unary(tokens);

        loop {
            if tokens.len() == 0 {
                break;
            }
            let token = &tokens[0];
            match token.operator {
                Some('*') => {
                    tokens.remove(0);
                    let rhs = Node::unary(tokens);
                    node = Node::operator('*', node, rhs);
                }
                Some('/') => {
                    tokens.remove(0);
                    let rhs = Node::unary(tokens);
                    node = Node::operator('/', node, rhs);
                }
                _ => {
                    break;
                }
            }
        }
        return node;
    }

    fn unary(tokens: &mut Vec<Token>) -> Self {
        let token = &tokens[0];
        match token.operator {
            Some('+') => {
                tokens.remove(0);
                return Node::term(tokens);
            }
            Some('-') => {
                tokens.remove(0);
                return Node::operator('-', Node::number(0), Node::term(tokens));
            }
            _ => {
                return Node::term(tokens);
            }
        }
    }

    fn term(tokens: &mut Vec<Token>) -> Self {
        if tokens[0].operator == Some('(') {
            let close_index = tokens
                .iter()
                .position(|token| token.operator == Some(')'))
                .unwrap();
            let mut exp = tokens[1..close_index].to_vec();
            tokens.drain(0..(close_index + 1));
            return Node::expr(&mut exp);
        } else {
            let num = tokens[0].value.unwrap();
            tokens.remove(0);
            return Node::number(num);
        };
    }
}
