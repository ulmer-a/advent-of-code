struct SevenSegmentLearner {
    segments_1: String,
    segments_7: String,
    segments_4: String,
}

impl SevenSegmentLearner {
    fn new() -> SevenSegmentLearner {
        SevenSegmentLearner {
            segments_1: String::new(),
            segments_7: String::new(),
            segments_4: String::new(),
        }
    }

    fn learn_word(&mut self, word: &str) {
        match word.len() {
            2 => self.segments_1 = word.into(),
            3 => self.segments_7 = word.into(),
            4 => self.segments_4 = word.into(),
            _ => {}
        }
    }

    fn recognize_number(&self, word: &str) -> i32 {
        match word.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            6 => {
                if Self::without(word, &self.segments_4).len() == 3 {
                    if Self::intersect(word, &self.segments_1).len() == 2 {
                        0
                    } else {
                        6
                    }
                } else {
                    9
                }
            }
            5 => {
                if Self::intersect(word, &self.segments_7).len() == 3 {
                    3
                } else if Self::without(word, &self.segments_4).len() == 3 {
                    2
                } else {
                    5
                }
            }
            _ => panic!("invalid"),
        }
    }

    fn intersect(s1: &str, s2: &str) -> String {
        let mut intersect_set = String::new();
        for c in s1.chars() {
            if s2.contains(c) {
                intersect_set.push(c);
            }
        }
        intersect_set
    }

    fn without(s1: &str, s2: &str) -> String {
        let mut intersect_set = String::new();
        for c in s1.chars() {
            if !s2.contains(c) {
                intersect_set.push(c);
            }
        }
        intersect_set
    }
}

pub fn main(input: String) -> (u64, u64) {
    let mut unique_len_digits = 0;
    let mut output_sum = 0;

    for line in input.lines() {
        let mut line = line.split(" | ");
        let train_words = line.next().unwrap();
        let out_words = line.next().unwrap();

        let mut learner = SevenSegmentLearner::new();
        for train_word in train_words.split_whitespace() {
            learner.learn_word(train_word);
        }

        let mut output_value = 0;
        for out_word in out_words.split_whitespace() {
            if let 2 | 3 | 4 | 7 = out_word.len() {
                unique_len_digits += 1
            }

            output_value = output_value * 10 + learner.recognize_number(out_word);
        }
        output_sum += output_value;
    }

    (unique_len_digits, output_sum as u64)
}

#[test]
fn test() {
    let input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;
    let r = main(input.into());
    assert_eq!(r.0, 26);
    assert_eq!(r.1, 61229);
}
