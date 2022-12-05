use std::collections::HashMap;

pub fn main(input: String) -> (u64, u64) {
    let (crates, moves) = input.split_once("\n\n").unwrap();
    let mut stacks1 = parse_crates(crates);
    let mut stacks2 = stacks1.clone();

    for line in moves.lines() {
        let m = Move::new(line);

        let v = stacks1.get_mut(&m.from).unwrap().take(m.n);
        stacks1.get_mut(&m.to).unwrap().put(v);
    }

    for line in moves.lines() {
        let m = Move::new(line);

        let v = stacks2.get_mut(&m.from).unwrap().take(m.n);
        stacks2.get_mut(&m.to).unwrap().put_9001(v);
    }

    (checksum(stacks1), checksum(stacks2))
}

fn checksum(stacks: HashMap<usize, CrateStack>) -> u64 {
    let mut checksum = 0;
    for i in 0..10 {
        match stacks.get(&i) {
            Some(s) => {
                let c = s.0.iter().last().unwrap();
                print!("{}", c);
                checksum += *c as u64;
            }
            None => break,
        }
    }
    println!();

    checksum
}

struct Move {
    n: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn new(line: &str) -> Self {
        let line = &line[5..];
        let (n, rest) = line.split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        Self {
            n: n.parse::<usize>().unwrap(),
            from: from.parse::<usize>().unwrap() - 1,
            to: to.parse::<usize>().unwrap() - 1,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct CrateStack(Vec<char>);

impl CrateStack {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn take(&mut self, n: usize) -> Vec<char> {
        let mut v = vec![];
        for _ in 0..n {
            v.push(self.0.pop().unwrap());
        }
        v
    }

    pub fn put(&mut self, v: Vec<char>) {
        for c in v {
            self.0.push(c);
        }
    }

    pub fn put_9001(&mut self, v: Vec<char>) {
        for c in v.iter().rev() {
            self.0.push(*c);
        }
    }
}

fn parse_crates(input: &str) -> HashMap<usize, CrateStack> {
    let stack_count = (input.lines().next().unwrap().len() + 1) / 4;

    let mut stacks = HashMap::new();
    for i in 0..stack_count {
        stacks.insert(i, CrateStack::new());
    }

    for line in input.lines().rev().skip(1) {
        for i in 0..stack_count {
            let stack = stacks.get_mut(&i).unwrap();
            match line.as_bytes()[1 + i * 4] {
                b' ' => {}
                c => stack.0.push(c as char),
            }
        }
    }

    stacks
}

#[test]
fn crate_parse_test() {
    let crates = r#"           
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "#;

    let mut stacks = HashMap::new();
    stacks.insert(0, CrateStack(vec!['Z', 'N']));
    stacks.insert(1, CrateStack(vec!['M', 'C', 'D']));
    stacks.insert(2, CrateStack(vec!['P']));

    assert_eq!(parse_crates(crates), stacks,);
}
