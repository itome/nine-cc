#[derive(Debug, Clone)]
pub struct Token {
    pub number: Option<i64>,
    pub operator: Option<String>,
    pub ident: Option<String>,
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
            ident: None,
        }
    }

    fn number(num: i64) -> Self {
        Token {
            number: Some(num),
            operator: None,
            ident: None,
        }
    }

    fn ident(ident: String) -> Self {
        Token {
            number: None,
            operator: None,
            ident: Some(ident),
        }
    }

    pub fn parse(input: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut input = input;

        loop {
            if input.is_empty() {
                break;
            }
            consume_whitespace(&mut input);
            if let Some(token) = consume_number(&mut input) {
                tokens.push(token);
                continue;
            }
            if let Some(token) = consume_operator(&mut input) {
                tokens.push(token);
                continue;
            }
            if let Some(token) = consume_ident(&mut input) {
                tokens.push(token);
                continue;
            }
        }

        return tokens;
    }
}

fn consume_whitespace(input: &mut String) {
    loop {
        match input.chars().next() {
            Some(c) if c.is_whitespace() => {
                input.remove(0);
            }
            _ => {
                break;
            }
        }
    }
}

fn consume_number(input: &mut String) -> Option<Token> {
    let mut digits = "".to_string();
    loop {
        match input.chars().next() {
            Some(c) if c.is_ascii_digit() => {
                digits += &c.to_string();
                input.remove(0);
            }
            _ => {
                break;
            }
        }
    }
    if digits.is_empty() {
        None
    } else {
        Some(Token::number(digits.parse::<i64>().unwrap()))
    }
}

fn consume_operator(input: &mut String) -> Option<Token> {
    if input.starts_with("return") {
        let token = Some(Token::operator(input[..6].to_string()));
        input.drain(0..6);
        return token;
    }
    if input.starts_with("==")
        || input.starts_with("!=")
        || input.starts_with("<=")
        || input.starts_with(">=")
    {
        let token = Some(Token::operator(input[..2].to_string()));
        input.drain(0..2);
        return token;
    }
    match input.chars().next() {
        Some(c)
            if c == '+'
                || c == '-'
                || c == '*'
                || c == '/'
                || c == '('
                || c == ')'
                || c == '>'
                || c == '<'
                || c == '='
                || c == ';' =>
        {
            input.remove(0);
            Some(Token::operator(c.to_string()))
        }
        _ => None,
    }
}

fn consume_ident(input: &mut String) -> Option<Token> {
    let mut chars = "".to_string();
    loop {
        match input.chars().next() {
            Some(c) if c.is_ascii_alphabetic() => {
                chars += &c.to_string();
                input.remove(0);
            }
            _ => {
                break;
            }
        }
    }
    if chars.is_empty() {
        None
    } else {
        Some(Token::ident(chars))
    }
}

#[cfg(test)]
mod tests {
    use crate::token::consume_ident;
    use crate::token::consume_number;
    use crate::token::consume_operator;
    use crate::token::consume_whitespace;
    use crate::token::Token;

    #[test]
    fn test_consume_whitespace() {
        let mut input = "  ".to_string();
        consume_whitespace(&mut input);
        assert_eq!(input, "".to_string());
        consume_whitespace(&mut input);
        assert_eq!(input, "".to_string());
    }

    #[test]
    fn test_consume_number() {
        let mut input = "12+".to_string();
        let output = consume_number(&mut input);
        assert_eq!(output, Some(Token::number(12)));
        assert_eq!(input, "+".to_string());
    }

    #[test]
    fn test_consume_ident() {
        let mut input = "ab12".to_string();
        let output = consume_ident(&mut input);
        assert_eq!(output, Some(Token::ident("ab".to_string())));
        assert_eq!(input, "12".to_string());
        let output = consume_ident(&mut input);
        assert_eq!(output, None);
        assert_eq!(input, "12".to_string());
    }

    #[test]
    fn test_consume_operator() {
        let mut input = "+12".to_string();
        let output = consume_operator(&mut input);
        assert_eq!(output, Some(Token::operator("+".to_string())));
        assert_eq!(input, "12".to_string());

        let mut input = "<=12".to_string();
        let output = consume_operator(&mut input);
        assert_eq!(output, Some(Token::operator("<=".to_string())));
        assert_eq!(input, "12".to_string());

        let mut input = "return 5".to_string();
        let output = consume_operator(&mut input);
        assert_eq!(output, Some(Token::operator("return".to_string())));
        assert_eq!(input, " 5".to_string());
    }

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
