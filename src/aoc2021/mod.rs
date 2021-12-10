mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn run(id: usize, input: String) -> Option<(u64, u64)> {
    Some(match id {
        1 => day1::main(input),
        2 => day2::main(input),
        3 => day3::main(input),
        4 => day4::main(input),
        5 => day5::main(input),
        6 => day6::main(input),
        7 => day7::main(input),
        8 => day8::main(input),
        9 => day9::main(input),
        10 => day10::main(input),
        _ => return None,
    })
}
