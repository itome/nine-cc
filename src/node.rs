use crate::token::Token;

#[derive(Debug)]
pub struct Node {
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub number: Option<i64>,
    pub operator: Option<String>,
    pub ident: Option<String>,
}

impl Node {
    fn operator(op: String, lhs: Node, rhs: Node) -> Self {
        Node {
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
            number: None,
            operator: Some(op),
            ident: None,
        }
    }

    fn number(num: i64) -> Self {
        Node {
            lhs: None,
            rhs: None,
            number: Some(num),
            operator: None,
            ident: None,
        }
    }

    fn ident(ident: String) -> Self {
        Node {
            lhs: None,
            rhs: None,
            number: None,
            operator: None,
            ident: Some(ident),
        }
    }

    pub fn program(tokens: &mut Vec<Token>) -> Vec<Self> {
        let mut nodes: Vec<Node> = vec!();
        while !tokens.is_empty() {
            nodes.push(Node::stmt(tokens));
        };
        return nodes;
    }

    fn stmt(tokens: &mut Vec<Token>) -> Self {
        let node = Node::expr(tokens);
        tokens.remove(0);
        return node;
    }

    fn expr(tokens: &mut Vec<Token>) -> Self {
        return Node::assign(tokens);
    }

    fn assign(tokens: &mut Vec<Token>) -> Self {
        let mut node = Node::equality(tokens);
        match &tokens.first() {
            Some(token) if token.operator == Some("=".to_string()) => {
                tokens.remove(0);
                let rhs = Node::assign(tokens);
                node = Node::operator("=".to_string(), node, rhs)
            }
            _ => {}
        }
        return node;
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
                        let rhs = Node::relational(tokens);
                        node = Node::operator("==".to_string(), node, rhs);
                    }
                    "!=" => {
                        tokens.remove(0);
                        let rhs = Node::relational(tokens);
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
                    "<" => {
                        tokens.remove(0);
                        let rhs = Node::mul(tokens);
                        node = Node::operator("<".to_string(), node, rhs);
                    }
                    "<=" => {
                        tokens.remove(0);
                        let rhs = Node::mul(tokens);
                        node = Node::operator("<=".to_string(), node, rhs);
                    }
                    ">" => {
                        tokens.remove(0);
                        let rhs = Node::mul(tokens);
                        node = Node::operator("<".to_string(), rhs, node);
                    }
                    ">=" => {
                        tokens.remove(0);
                        let rhs = Node::mul(tokens);
                        node = Node::operator("<=".to_string(), rhs, node);
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
            Some(op) if op == "(" => {
                let close_index = tokens
                    .iter()
                    .position(|token| token.operator == Some(")".to_string()))
                    .unwrap();
                let mut exp = tokens[1..close_index].to_vec();
                tokens.drain(0..(close_index + 1));
                return Node::expr(&mut exp);
            }
            _ => match &tokens[0].ident {
                Some(ident) => {
                    let ident = ident.clone();
                    tokens.remove(0);
                    return Node::ident(ident);
                }
                _ => {
                    let num = tokens[0].number.unwrap();
                    tokens.remove(0);
                    return Node::number(num);
                }
            },
        }
    }
}
