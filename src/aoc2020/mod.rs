mod day1;
mod day18;

pub fn run(id: usize, input: String) -> Option<(u64, u64)> {
    Some(match id {
        1 => day1::main(input),
        18 => day18::main(input),
        _ => return None,
    })
}
