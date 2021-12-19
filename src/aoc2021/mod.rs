mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

pub fn run(id: usize, input: String) -> Option<(u64, u64)> {
    Some(match id {
        1 => day01::main(input),
        2 => day02::main(input),
        3 => day03::main(input),
        4 => day04::main(input),
        5 => day05::main(input),
        6 => day06::main(input),
        7 => day07::main(input),
        8 => day08::main(input),
        9 => day09::main(input),
        10 => day10::main(input),
        11 => day11::main(input),
        12 => day12::main(input),
        13 => day13::main(input),
        14 => day14::main(input),
        15 => day15::main(input),
        16 => day16::main(input),
        17 => day17::main(input),
        18 => day18::main(input),
        19 => day19::main(input),
        _ => return None,
    })
}
