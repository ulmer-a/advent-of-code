use pair::Pair;

pub fn main(input: String) -> (u64, u64) {
    let magnitude = input
        .lines()
        .map(|l| Pair::new(l))
        .fold(Pair::neutral(), |p1, p2| p1 + p2)
        .magnitude();

    let mut max_magnitude = 0;
    for a in input.lines().map(|l| Pair::new(l)) {
        for b in input.lines().map(|l| Pair::new(l)) {
            let mag = (a.clone() + b).magnitude();
            if mag > max_magnitude {
                max_magnitude = mag;
            }
        }
    }

    (magnitude, max_magnitude)
}

mod pair {
    use std::ops::Add;

    use logos::{Lexer, Logos};

    #[derive(Logos, Debug, PartialEq)]
    enum Token {
        #[token("[")]
        SqBracketOpen,

        #[token("]")]
        SqBracketClose,

        #[token(",")]
        Comma,

        #[regex("[0-9]+")]
        Number,

        #[error]
        #[regex(r"[ \t\n\f]+", logos::skip)]
        Error,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct Pair(String);

    impl Pair {
        pub fn new(source: &str) -> Pair {
            Pair(source.into())
        }

        pub fn neutral() -> Pair {
            Pair("".into())
        }

        pub fn magnitude(self) -> u64 {
            fn mag_recursive(lexer: &mut Lexer<Token>) -> u64 {
                match lexer.next().unwrap() {
                    Token::Number => lexer.slice().parse::<u64>().unwrap(),
                    Token::SqBracketOpen => {
                        let m1 = mag_recursive(lexer);
                        Pair::accept_token(lexer, Token::Comma);
                        let m2 = mag_recursive(lexer);
                        Pair::accept_token(lexer, Token::SqBracketClose);
                        3 * m1 + 2 * m2
                    }
                    t => panic!("magnitude: unexpected token: {:?}", t),
                }
            }

            let mut lexer: Lexer<Token> = Lexer::new(self.0.as_str());
            mag_recursive(&mut lexer)
        }

        pub fn split(self) -> Result<Self, Self> {
            let mut done = false;
            let mut out = String::new();
            let mut lexer: Lexer<Token> = Lexer::new(self.0.as_str());
            while let Some(token) = &mut lexer.next() {
                match token {
                    Token::Number if !done => {
                        let n = lexer.slice().parse::<i32>().unwrap();
                        if n >= 10 {
                            out.push_str(&format!("[{},{}]", n / 2, (n + 1) / 2));
                            done = true;
                        } else {
                            out.push_str(lexer.slice());
                        }
                    }
                    _ => out.push_str(lexer.slice()),
                }
            }
            match done {
                true => Ok(Pair(out)),
                false => Err(Pair(out)),
            }
        }

        pub fn reduce(self) -> Self {
            match self.explode() {
                Ok(p) => p.reduce(),
                Err(p) => match p.split() {
                    Ok(p) => p.reduce(),
                    Err(p) => p,
                },
            }
        }

        pub fn explode(self) -> Result<Self, Self> {
            let mut done = None;
            let mut level = 0;
            let mut number_counter = 0;
            let mut out = String::new();
            let mut lexer: Lexer<Token> = Lexer::new(self.0.as_str());
            while let Some(token) = &mut lexer.next() {
                match token {
                    Token::SqBracketOpen if done.is_none() && level >= 4 => {
                        let (n1, n2) = Self::accept_tuple(&mut lexer);
                        Self::accept_token(&mut lexer, Token::SqBracketClose);
                        done =
                            Some(((n1, number_counter), (n2, number_counter + 2)));
                        out.push_str("0");
                    }
                    Token::Number => {
                        number_counter += 1;
                        out.push_str(lexer.slice());
                    }
                    Token::SqBracketClose => {
                        level -= 1;
                        out.push_str("]");
                    }
                    Token::SqBracketOpen => {
                        level += 1;
                        out.push_str("[");
                    }
                    _ => {
                        out.push_str(lexer.slice());
                    }
                }
            }

            fn add_to_nth_number(s: String, n: usize, value: i32) -> String {
                let mut lexer: Lexer<Token> = Lexer::new(s.as_str());
                let mut counter = 0;
                let mut out = String::new();
                while let Some(token) = &mut lexer.next() {
                    match token {
                        Token::Number if counter == n => {
                            counter += 1;
                            let n = lexer.slice().parse::<i32>().unwrap();
                            out.push_str(&format!("{}", n + value));
                        }
                        Token::Number => {
                            counter += 1;
                            out.push_str(lexer.slice());
                        }
                        _ => {
                            out.push_str(lexer.slice());
                        }
                    }
                }
                out
            }

            match done {
                Some((n1, n2)) => {
                    if n1.1 != 0 {
                        out = add_to_nth_number(out, n1.1 - 1, n1.0);
                        out = add_to_nth_number(out, n2.1 - 1, n2.0);
                    } else {
                        out = add_to_nth_number(out, n2.1 - 1, n2.0);
                    }
                    Ok(Pair(out))
                }
                None => Err(Pair(out)),
            }
        }

        fn accept_token(lexer: &mut Lexer<Token>, token: Token) {
            assert_eq!(lexer.next().unwrap(), token);
        }

        fn accept_tuple(lexer: &mut Lexer<Token>) -> (i32, i32) {
            Self::accept_token(lexer, Token::Number);
            let n1 = lexer.slice().parse::<i32>().unwrap();
            Self::accept_token(lexer, Token::Comma);
            Self::accept_token(lexer, Token::Number);
            let n2 = lexer.slice().parse::<i32>().unwrap();
            (n1, n2)
        }
    }

    impl Add for Pair {
        type Output = Pair;

        fn add(self, rhs: Self) -> Self::Output {
            match (self.0, rhs.0) {
                (lhs, s) if s.is_empty() => Pair(lhs),
                (s, rhs) if s.is_empty() => Pair(rhs),
                (lhs, rhs) => Pair::new(&format!("[{},{}]", lhs, rhs)).reduce(),
            }
        }
    }
}

#[test]
fn test() {
    let input = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;

    let r = main(input.into());
    assert_eq!(r.0, 4140);
    assert_eq!(r.1, 3993);
}

#[test]
fn explode() {
    assert_eq!(
        Pair::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
            .explode()
            .unwrap(),
        Pair::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
    );
    assert_eq!(
        Pair::new("[[[[[9,8],1],2],3],4]").explode().unwrap(),
        Pair::new("[[[[0,9],2],3],4]"),
    );
    assert_eq!(
        Pair::new("[7,[6,[5,[4,[3,2]]]]]").explode().unwrap(),
        Pair::new("[7,[6,[5,[7,0]]]]"),
    );
}

#[test]
fn split() {
    assert_eq!(
        Pair::new("[[[[0,7],4],[15,[0,13]]],[1,1]]")
            .split()
            .unwrap(),
        Pair::new("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")
    );
    assert_eq!(
        Pair::new("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")
            .split()
            .unwrap(),
        Pair::new("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")
    );
}

#[test]
fn addition() {
    assert_eq!(
        Pair::new("[1,2]") + Pair::new("[[3,4],5]"),
        Pair::new("[[1,2],[[3,4],5]]"),
    );
}

#[test]
fn magnitude() {
    assert_eq!(Pair::new("[[1,2],[[3,4],5]]").magnitude(), 143);
    assert_eq!(
        Pair::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(),
        1384
    );
    assert_eq!(Pair::new("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
    assert_eq!(Pair::new("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
    assert_eq!(Pair::new("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(), 1137);
    assert_eq!(
        Pair::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
            .magnitude(),
        3488
    );
}
