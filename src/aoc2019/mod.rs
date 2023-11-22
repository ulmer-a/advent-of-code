mod day01;

pub fn run(id: usize, input: String) -> Option<(u64, u64)> {
    match id {
        1 => Some(day01::main(input)),
        _ => None,
    }
}
