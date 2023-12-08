use std::fs;
use day01::part1::process;

fn main() -> Result<(), String> {
    let file = fs::read_to_string("d1p1.txt").unwrap();
    let result = process(&file).unwrap();
    println!("Part1: {result}");

    Ok(())
}
