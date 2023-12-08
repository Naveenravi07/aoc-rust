use std::fs;
use day01::part2::process;

fn main() -> Result<(), String> {
    let file = fs::read_to_string("d1p2.txt").unwrap();
    let result = process(&file).unwrap();
    println!("Part2: {result}");

    Ok(())
}

