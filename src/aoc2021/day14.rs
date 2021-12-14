use std::collections::HashMap;

pub fn main(input: String) -> (u64, u64) {
    let template = input.split("\n\n").next().unwrap();
    let rules = parse_rules(&input);

    (
        polymer_insert(template, &rules, 10),
        polymer_insert(template, &rules, 40),
    )
}

fn polymer_insert(template: &str, rules: &HashMap<(char, char), char>, depth: usize) -> u64 {
    let mut dyn_lookup_map = HashMap::new();
    for (key, _) in rules.iter() {
        dyn_lookup_map.insert(key, [0u64; 26]);
    }

    for _ in 0..depth {
        // for each level of recursion, build a map of letter
        // counts for each possible map key.
        let mut next_map = HashMap::new();
        for (rule, next_c) in rules.iter() {
            let letter_counts = sum_letter_count(
                *next_c,
                dyn_lookup_map.get(&(rule.0, *next_c)).unwrap(),
                dyn_lookup_map.get(&(*next_c, rule.1)).unwrap(),
            );
            next_map.insert(rule, letter_counts);
        }
        dyn_lookup_map = next_map;
    }

    // finally, apply the results in the top level and get the solution
    let mut letter_counts = [0u64; 26];
    let mut prev_c = template.chars().next().unwrap();
    for c in template.chars().skip(1) {
        let path_counts = dyn_lookup_map.get(&(prev_c, c)).unwrap();
        letter_counts = sum_letter_count(prev_c, &letter_counts, path_counts);
        prev_c = c;
    }
    letter_counts[prev_c as usize - 'A' as usize] += 1;

    // extract most and least common character counts
    let least_common = letter_counts.iter().filter(|c| *c > &0).min().unwrap();
    let most_common = letter_counts.iter().max().unwrap();
    most_common - least_common
}

/// element-wise addition of l1, l2 as well as addition of char c
fn sum_letter_count(c: char, l1: &[u64; 26], l2: &[u64; 26]) -> [u64; 26] {
    let mut letter_counts = *l1;
    for i in 0..26 {
        letter_counts[i] += l2[i];
    }
    letter_counts[c as usize - 'A' as usize] += 1;
    letter_counts
}

fn parse_rules(source: &String) -> HashMap<(char, char), char> {
    let mut rules = HashMap::new();
    for rule in source.lines().skip(2) {
        let mut it = rule.split(" -> ");
        let mut key_chars = it.next().unwrap().chars();
        rules.insert(
            (key_chars.next().unwrap(), key_chars.next().unwrap()),
            it.next().unwrap().chars().next().unwrap(),
        );
    }
    rules
}

#[test]
fn test_main() {
    let input = r#"NNCB

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
    let r = main(input.into());
    assert_eq!(r.0, 1588);
    assert_eq!(r.1, 2188189693529);
}
