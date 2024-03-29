mod day01;
mod day02;

pub fn run(id: usize, input: String) -> Option<(u64, u64)> {
    Some(match id {
        1 => day01::main(input),
        2 => day02::main(input),
        _ => return None,
    })
}
