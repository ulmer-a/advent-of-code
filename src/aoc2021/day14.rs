use std::collections::HashMap;

pub fn main(input: String) -> (u64, u64) {
    let mut formula = String::from(input.split("\n\n").next().unwrap());
    let rules = parse_rules(input);

    for _ in 0..10 {
        formula = iter_polymer(formula, &rules);
    }

    (measure(&formula), 0)
}

fn measure(s: &String) -> u64 {
    let mut counts = [0; 26];
    for c in s.bytes() {
        if c >= 'A' as u8 && c <= 'Z' as u8 {
            let g = &mut counts[c as usize - 'A' as usize];
            *g += 1;
        }
    }
    let max_value = counts.iter().max().unwrap();
    let min_value = counts.iter().filter(|n| *n > &0).min().unwrap();
    (max_value - min_value) as u64
}

fn iter_polymer(formula: String, rules: &HashMap<String, char>) -> String {
    let mut next_formula = String::new();
    let mut prev_c = formula.chars().next().unwrap();
    for c in formula.chars().skip(1) {
        next_formula.push(prev_c);
        if let Some(plug_c) = rules.get(&format!("{}{}", prev_c, c)) {
            next_formula.push(*plug_c);
        }
        prev_c = c;
    }
    next_formula.push(prev_c);
    next_formula
}

fn parse_rules(source: String) -> HashMap<String, char> {
    let mut rules = HashMap::new();
    for rule in source.lines().skip(2) {
        let mut it = rule.split(" -> ");
        rules.insert(
            it.next().unwrap().into(),
            it.next().unwrap().chars().next().unwrap(),
        );
    }
    rules
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    #[test]
    fn iter_test() {
        let mut formula = String::from(INPUT.split("\n\n").next().unwrap());
        let rules = super::parse_rules(INPUT.into());

        formula = super::iter_polymer(formula, &rules);
        assert_eq!(formula, "NCNBCHB");

        formula = super::iter_polymer(formula, &rules);
        assert_eq!(formula, "NBCCNBBBCBHCB");

        formula = super::iter_polymer(formula, &rules);
        assert_eq!(formula, "NBBBCNCCNBBNBNBBCHBHHBCHB");

        formula = super::iter_polymer(formula, &rules);
        assert_eq!(formula, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
    }

    #[test]
    fn test_main() {
        let r = super::main(INPUT.into());
        assert_eq!(r.0, 1588);
    }
}
