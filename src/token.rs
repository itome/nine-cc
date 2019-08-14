use std::process::exit;

#[derive(Debug, Clone)]
pub struct Token {
    pub number: Option<i64>,
    pub operator: Option<String>,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.operator == other.operator
    }
}

impl Token {
    fn operator(op: String) -> Self {
        Token {
            number: None,
            operator: Some(op),
        }
    }

    fn number(num: i64) -> Self {
        Token {
            number: Some(num),
            operator: None,
        }
    }

    pub fn parse(input: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut input = input.clone();
        let mut current_token = String::from("");
        while let Some(c) = input.chars().nth(0) {
            if c.is_whitespace() {
                input = input.split_off(1);
                continue;
            }

            if (current_token == ">" || current_token == "<") && c != '=' {
                let token = Token {
                    number: None,
                    operator: Some(current_token.clone() + &c.to_string()),
                };
                current_token = String::from("");
                tokens.push(token);
            }

            if c == '=' && current_token.len() > 0 {
                let token = Token {
                    number: None,
                    operator: Some(current_token.clone() + &c.to_string()),
                };
                current_token = String::from("");
                tokens.push(token);
                continue;
            }

            if c == '=' || c == '!' || c == '<' || c == '>' {
                current_token = c.to_string();
                continue;
            }

            if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' {
                let token = Token {
                    number: None,
                    operator: Some(c.to_string()),
                };
                input = input.split_off(1);
                tokens.push(token);
                continue;
            }

            if c.is_ascii_digit() {
                let (num, remaining) = strtol(&input);
                input = remaining;
                let token = Token {
                    number: num,
                    operator: None,
                };
                tokens.push(token);
                continue;
            }

            eprintln!("cannot tokenize: {}", c);
            exit(1);
        }

        tokens.push(Token {
            number: None,
            operator: None,
        });

        return tokens;
    }
}

fn strtol(s: &String) -> (Option<i64>, String) {
    if s.is_empty() {
        return (None, s.clone());
    }

    let mut pos = 0;
    let mut remaining = s.clone();
    let len = s.len();

    while len != pos {
        if !s.chars().nth(pos).unwrap().is_ascii_digit() {
            break;
        }
        pos += 1;
    }

    if len == pos {
        (Some(remaining.parse::<i64>().unwrap()), "".into())
    } else {
        let t: String = remaining.drain(..pos).collect();
        (Some(t.parse::<i64>().unwrap()), remaining)
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    #[test]
    fn plus() {
        let input = "1 + 4";
        let output = Token::parse(input.to_string());
        assert_eq!(output[0], Token::number(1));
        assert_eq!(output[1], Token::operator("+".to_string()));
        assert_eq!(output[2], Token::number(4));
    }

    #[test]
    fn minus() {
        let input = "3 - 2";
        let output = Token::parse(input.to_string());
        assert_eq!(output[0], Token::number(3));
        assert_eq!(output[1], Token::operator("-".to_string()));
        assert_eq!(output[2], Token::number(2));
    }

    #[test]
    fn mul() {
        let input = "4 * 4";
        let output = Token::parse(input.to_string());
        assert_eq!(output[0], Token::number(4));
        assert_eq!(output[1], Token::operator("*".to_string()));
        assert_eq!(output[2], Token::number(4));
    }

    #[test]
    fn div() {
        let input = "4 / 4";
        let output = Token::parse(input.to_string());
        assert_eq!(output[0], Token::number(4));
        assert_eq!(output[1], Token::operator("/".to_string()));
        assert_eq!(output[2], Token::number(4));
    }

    #[test]
    fn plus_and_mul() {
        let input = "1 + 4 * 4 - 1";
        let output = Token::parse(input.to_string());
        assert_eq!(output[0], Token::number(1));
        assert_eq!(output[1], Token::operator("+".to_string()));
        assert_eq!(output[2], Token::number(4));
        assert_eq!(output[3], Token::operator("*".to_string()));
        assert_eq!(output[4], Token::number(4));
        assert_eq!(output[5], Token::operator("-".to_string()));
        assert_eq!(output[6], Token::number(1));
    }

    #[test]
    fn brackets() {
        let input = "(1 + 4) * 2";
        let output = Token::parse(input.to_string());
        assert_eq!(output[0], Token::operator("(".to_string()));
        assert_eq!(output[1], Token::number(1));
        assert_eq!(output[2], Token::operator("+".to_string()));
        assert_eq!(output[3], Token::number(4));
        assert_eq!(output[4], Token::operator(")".to_string()));
        assert_eq!(output[5], Token::operator("*".to_string()));
        assert_eq!(output[6], Token::number(2));
    }

    #[test]
    fn two_digit() {
        let input = "15 + 40";
        let output = Token::parse(input.to_string());
        assert_eq!(output[0], Token::number(15));
        assert_eq!(output[1], Token::operator("+".to_string()));
        assert_eq!(output[2], Token::number(40));
    }
}
