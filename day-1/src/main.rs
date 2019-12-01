use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    println!(
        "{}",
        input
            .lines()
            // Use saturating sub to avoid wrapping.
            .map(|line| (line.parse::<usize>().unwrap() / 3).saturating_sub(2))
            .sum::<usize>()
    );
}

fn part_2(input: &str) {
    println!(
        "{}",
        input.lines()
            .map(|line| get_additional_fuel((line.parse::<usize>().unwrap() / 3).saturating_sub(2)))
            .sum::<usize>()
    );
}

fn get_additional_fuel(fuel: usize) -> usize {
    // If the amount of additional fuel would be 0 or negative, just return the current amount of
    // additional fuel.
    if fuel / 3 <= 2 {
        fuel
    } else {
        // Otherwise, find out how much more we need.
        fuel + get_additional_fuel(fuel / 3 - 2)
    }
}
