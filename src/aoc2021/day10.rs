pub fn main(input: String) -> (u64, u64) {
    let mut syntax_score = 0;
    let mut autocomplete_scores = vec![];

    for line in input.lines() {
        let mut chunk_stack = vec![];
        for c in line.chars() {
            match c {
                '(' => chunk_stack.push(')'),
                '[' => chunk_stack.push(']'),
                '{' => chunk_stack.push('}'),
                '<' => chunk_stack.push('>'),
                other => {
                    if other != chunk_stack.pop().unwrap() {
                        // corruption has been found
                        syntax_score += match other {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0,
                        };

                        // clear the stack and break to discard this line
                        chunk_stack.clear();
                        break;
                    }
                }
            }
        }

        if chunk_stack.len() > 0 {
            let score = autocomplete_line_score(&mut chunk_stack);
            autocomplete_scores.push(score);
        }
    }

    autocomplete_scores.sort();
    let middle = autocomplete_scores.len() / 2;
    (syntax_score, autocomplete_scores[middle])
}

fn autocomplete_line_score(chunk_stack: &mut Vec<char>) -> u64 {
    let mut score: u64 = 0;
    while let Some(leftover) = chunk_stack.pop() {
        score = score * 5
            + match leftover {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            }
    }
    score
}

#[test]
fn test() {
    let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;
    let r = main(input.into());
    assert_eq!(r.0, 26397);
    assert_eq!(r.1, 288957);
}
