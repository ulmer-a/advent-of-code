use itertools::Itertools;

pub fn main(input: String) -> (u64, u64) {
    let elf_calories: Vec<u64> = input
        .split("\n\n")
        .map(|block| block.lines().map(|line| line.parse::<u64>().unwrap()).sum())
        .sorted()
        .collect();

    let max_elf_calories = elf_calories.last().unwrap();
    let max_3_elf_calories = elf_calories.iter().rev().take(3).sum();

    (*max_elf_calories, max_3_elf_calories)
}
