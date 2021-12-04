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

pub fn main(input: String) {
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
}
