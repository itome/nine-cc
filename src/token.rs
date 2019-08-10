use std::process::exit;

#[derive(Debug, Clone)]
pub struct Token {
    pub value: Option<i64>,
    pub operator: Option<char>,
}

impl Token {
    pub fn parse(input: String) -> Vec<Token> {
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