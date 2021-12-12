use std::collections::HashMap;

pub fn main(input: String) -> (u64, u64) {
    let adjacency_map = gen_adjacency_map(input);
    let mut visited_caves = vec![];

    (
        visit_cave("start", &adjacency_map, &mut visited_caves, 1),
        visit_cave("start", &adjacency_map, &mut visited_caves, 2),
    )
}

fn gen_adjacency_map(input: String) -> HashMap<String, Vec<String>> {
    let mut adjacency_map: HashMap<String, Vec<String>> = HashMap::new();

    let mut adj_insert = |s1: &str, s2: &str| {
        if let Some(vec) = adjacency_map.get_mut(s1) {
            vec.push(s2.into());
        } else {
            adjacency_map.insert(s1.into(), vec![s2.into()]);
        }
    };

    for line in input.lines() {
        let mut line = line.split("-");
        let s1 = line.next().unwrap();
        let s2 = line.next().unwrap();

        adj_insert(s1, s2);
        adj_insert(s2, s1);
    }

    adjacency_map
}

fn is_small_cave(s: &str) -> bool {
    s.chars().next().unwrap().is_lowercase()
}

fn has_been_visited(visited: &Vec<String>, cave: &String, mode: &mut u8) -> bool {
    let result = visited.contains(cave);
    if *mode == 2 && cave != "start" && result {
        *mode = 1;
        false
    } else {
        result
    }
}

// fn print_visited(v: &Vec<String>) {
//     let mut str = String::new();
//     for s in v {
//         str.push_str(&format!("{} -> ", s));
//     }
//     println!("{} end", str);
// }

fn visit_cave(
    current: &str,
    adj: &HashMap<String, Vec<String>>,
    visited: &mut Vec<String>,
    mode: u8,
) -> u64 {
    if current == "end" {
        //print_visited(visited);
        return 1;
    }

    let mut paths = 0;
    visited.push(current.into());
    if let Some(adj_list) = adj.get(current) {
        for next_cave in adj_list {
            let mut mode = mode;
            if is_small_cave(next_cave) && has_been_visited(visited, next_cave, &mut mode) {
                continue;
            }

            paths += visit_cave(next_cave, adj, visited, mode);
        }
    }
    visited.pop();
    paths
}

#[test]
fn test1() {
    let input = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
    let r = main(input.into());
    assert_eq!(r.0, 10);
    assert_eq!(r.1, 36);
}

#[test]
fn test2() {
    let input = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;
    let r = main(input.into());
    assert_eq!(r.0, 19);
    assert_eq!(r.1, 103);
}

#[test]
fn test3() {
    let input = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;
    let r = main(input.into());
    assert_eq!(r.0, 226);
    assert_eq!(r.1, 3509);
}
