use std::collections::HashMap;

struct BingoBoard {
    numbers: Vec<u32>,
    number_map: HashMap<u32, bool>,
}

impl BingoBoard {
    fn from_str(bingo_str: &str) -> BingoBoard {
        let mut numbers = vec![];
        let mut number_map = HashMap::new();
        for number_str in bingo_str.split_whitespace() {
            let n = number_str.parse::<u32>().unwrap();
            numbers.push(n);
            number_map.insert(n, false);
        }
        BingoBoard {
            numbers,
            number_map,
        }
    }

    pub fn cross_off(&mut self, n: u32) {
        if let Some(status) = self.number_map.get_mut(&n) {
            *status = true;
        }
    }

    pub fn wins(&self) -> Option<u32> {
        for row in 0..5 {
            if self.row_wins(row) {
                return Some(self.score());
            }
        }

        for col in 0..5 {
            if self.col_wins(col) {
                return Some(self.score());
            }
        }

        None
    }

    fn score(&self) -> u32 {
        let mut sum = 0;
        for (n, crossed) in self.number_map.iter() {
            if !crossed {
                sum += n;
            }
        }
        sum
    }

    fn row_wins(&self, r: usize) -> bool {
        for i in 0..5 {
            let n = self.numbers[r * 5 + i];
            if !self.number_map.get(&n).unwrap() {
                return false;
            }
        }
        true
    }

    fn col_wins(&self, c: usize) -> bool {
        for r in 0..5 {
            let n = self.numbers[r * 5 + c];
            if !self.number_map.get(&n).unwrap() {
                return false;
            }
        }
        true
    }
}

fn parse_random_numbers(line: &str) -> Vec<u32> {
    line.split(",").map(|n| n.parse::<u32>().unwrap()).collect()
}

pub fn main(input: String) -> (u64, u64) {
    let random_numbers = parse_random_numbers(input.lines().next().unwrap());

    let mut boards: Vec<BingoBoard> = vec![];
    for board_str in input.split("\n\n").skip(1) {
        boards.push(BingoBoard::from_str(board_str));
    }

    let (mut min_t, mut score_max) = (u32::MAX, 0);
    let (mut max_t, mut score_min) = (u32::MIN, 0);
    for board in boards.iter_mut() {
        let mut t = 0;
        for n in random_numbers.iter() {
            board.cross_off(*n);

            if let Some(board_score) = board.wins() {
                if t < min_t {
                    min_t = t;
                    score_max = board_score * n;
                }
                if t > max_t {
                    max_t = t;
                    score_min = board_score * n;
                }
                break;
            }

            t += 1;
        }
    }

    println!("first win in {} steps with score {}", min_t, score_max);
    println!("last win in {} steps with score {}", max_t, score_min);

    (score_max as u64, score_min as u64)
}

#[test]
fn test() {
    let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"#;

    let result = main(input.into());
    assert_eq!(result.0, 4512);
    assert_eq!(result.1, 1924);
}
