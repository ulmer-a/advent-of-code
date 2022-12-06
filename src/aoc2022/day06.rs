pub fn main(input: String) -> (u64, u64) {
    let start_of_packet = find_marker(&input, 4);
    let start_of_msg = find_marker(&input, 14);
    (start_of_packet as u64, start_of_msg as u64)
}

fn find_marker(input: &str, n: usize) -> usize {
    let mut window;
    'outer: for i in n..input.len() {
        window = &input[(i - n)..i];

        for c1 in window.chars() {
            if window.chars().map(|c2| (c1 == c2) as usize).sum::<usize>() != 1 {
                continue 'outer;
            }
        }

        return i;
    }
    panic!("not found")
}
