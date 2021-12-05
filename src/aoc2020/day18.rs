#[derive(PartialEq, Clone)]
enum Token {
    OperatorAdd,
    OperatorMul,
    ParensOpen,
    ParensClose,
    Number(u64),
    End,
}

struct Lexer<'a> {
    line: &'a str,
    last_token: Token,
}

impl<'a> Lexer<'a> {
    fn new(source: &str) -> Lexer {
        Lexer {
            line: source,
            last_token: Token::End,
        }
    }

    fn next_token(&mut self) -> Token {
        self.last_token = self.do_next_token();
        self.last_token.clone()
    }

    fn do_next_token(&mut self) -> Token {
        if let Some(index) = self.line.find(|c: char| !c.is_whitespace()) {
            self.line = &self.line[index..];
        } else {
            return Token::End;
        }

        if let Some(token) = match self.line.chars().next().unwrap() {
            '(' => Some(Token::ParensOpen),
            ')' => Some(Token::ParensClose),
            '+' => Some(Token::OperatorAdd),
            '*' => Some(Token::OperatorMul),
            _ => None,
        } {
            self.line = &self.line[1..];
            return token;
        }

        let mut number_str = self.line;
        if let Some(index) = self.line.find(|c: char| !c.is_digit(10)) {
            number_str = &self.line[..index];
        }

        self.line = &self.line[number_str.len()..];
        if let Ok(n) = number_str.parse::<u64>() {
            return Token::Number(n);
        }

        panic!("syntax error, unexpected characters");
    }
}

fn parse_primary(lexer: &mut Lexer) -> u64 {
    match lexer.next_token() {
        Token::Number(n) => n,
        Token::ParensOpen => parse_expr(lexer),
        _ => panic!("syntax error: expected number"),
    }
}

fn parse_expr(lexer: &mut Lexer) -> u64 {
    let mut operand = parse_primary(lexer);
    let mut operator = lexer.next_token();
    while operator != Token::End && operator != Token::ParensClose {
        match operator {
            Token::OperatorAdd => operand += parse_primary(lexer),
            Token::OperatorMul => operand *= parse_primary(lexer),
            _ => panic!("syntax error: unexpected token"),
        }
        operator = lexer.next_token();
    }
    operand
}

pub fn main(input: String) -> (u64, u64) {
    let mut sum = 0;
    for line in input.lines() {
        let mut lexer = Lexer::new(line);
        sum += parse_expr(&mut lexer);
    }
    (sum, 0)
}

#[test]
fn test() {
    let input = "1 + (2 * 3) + (4 * (5 + 6))";
    let result = main(input.into());
    assert_eq!(result.0, 51);
}
