use crate::token::Token;

#[derive(Debug)]
pub struct Node {
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub number: Option<i64>,
    pub operator: Option<String>,
}

impl Node {
    fn operator(op: String, lhs: Node, rhs: Node) -> Self {
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
        return Node::equality(tokens);
    }

    fn equality(tokens: &mut Vec<Token>) -> Self {
        let mut node = Node::relational(tokens);

        loop {
            if tokens.len() == 0 {
                break;
            }
            let token = &tokens[0];
            match &token.operator {
                Some(op) => match op.as_ref() {
                    "==" => {
                        tokens.remove(0);
                        let rhs = Node::mul(tokens);
                        node = Node::operator("==".to_string(), node, rhs);
                    }
                    "!=" => {
                        tokens.remove(0);
                        let rhs = Node::mul(tokens);
                        node = Node::operator("!=".to_string(), node, rhs);
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

    fn relational(tokens: &mut Vec<Token>) -> Self {
        let mut node = Node::add(tokens);

        loop {
            if tokens.len() == 0 {
                break;
            }
            let token = &tokens[0];
            match &token.operator {
                Some(op) => match op.as_ref() {
                    "<=" => {
                        tokens.remove(0);
                        let rhs = Node::mul(tokens);
                        node = Node::operator("<=".to_string(), node, rhs);
                    }
                    ">=" => {
                        tokens.remove(0);
                        let rhs = Node::mul(tokens);
                        node = Node::operator(">=".to_string(), node, rhs);
                    }
                    "<" => {
                        tokens.remove(0);
                        let rhs = Node::mul(tokens);
                        node = Node::operator("<".to_string(), node, rhs);
                    }
                    ">" => {
                        tokens.remove(0);
                        let rhs = Node::mul(tokens);
                        node = Node::operator(">".to_string(), node, rhs);
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

    fn add(tokens: &mut Vec<Token>) -> Self {
        let mut node = Node::mul(tokens);

        loop {
            if tokens.len() == 0 {
                break;
            }
            let token = &tokens[0];
            match &token.operator {
                Some(op) => match op.as_ref() {
                    "+" => {
                        tokens.remove(0);
                        let rhs = Node::mul(tokens);
                        node = Node::operator("+".to_string(), node, rhs);
                    }
                    "-" => {
                        tokens.remove(0);
                        let rhs = Node::mul(tokens);
                        node = Node::operator("-".to_string(), node, rhs);
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

    fn mul(tokens: &mut Vec<Token>) -> Self {
        let mut node = Node::unary(tokens);

        loop {
            if tokens.len() == 0 {
                break;
            }
            let token = &tokens[0];
            match &token.operator {
                Some(op) => match op.as_ref() {
                    "*" => {
                        tokens.remove(0);
                        let rhs = Node::unary(tokens);
                        node = Node::operator("*".to_string(), node, rhs);
                    }
                    "/" => {
                        tokens.remove(0);
                        let rhs = Node::unary(tokens);
                        node = Node::operator("/".to_string(), node, rhs);
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

    fn unary(tokens: &mut Vec<Token>) -> Self {
        let token = &tokens[0];
        match &token.operator {
            Some(op) => match op.as_ref() {
                "+" => {
                    tokens.remove(0);
                    return Node::term(tokens);
                }
                "-" => {
                    tokens.remove(0);
                    return Node::operator("-".to_string(), Node::number(0), Node::term(tokens));
                }
                _ => {
                    return Node::term(tokens);
                }
            },
            _ => {
                return Node::term(tokens);
            }
        }
    }

    fn term(tokens: &mut Vec<Token>) -> Self {
        match &tokens[0].operator {
            Some(op) => match op.as_ref() {
                "(" => {
                    let close_index = tokens
                        .iter()
                        .position(|token| token.operator == Some(")".to_string()))
                        .unwrap();
                    let mut exp = tokens[1..close_index].to_vec();
                    tokens.drain(0..(close_index + 1));
                    return Node::expr(&mut exp);
                }
                _ => {
                    let num = tokens[0].value.unwrap();
                    tokens.remove(0);
                    return Node::number(num);
                }
            },
            _ => {
                let num = tokens[0].value.unwrap();
                tokens.remove(0);
                return Node::number(num);
            }
        }
    }
}
