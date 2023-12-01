use std::{fs, io};

pub fn process(input: &str) -> Result<String, io::Error> {
    let file_content: String = fs::read_to_string(input)?;

    let sum: u32 = file_content
        .lines()
        .map(|line| {
            let mut iterator = line.chars().filter_map(|character| character.to_digit(10));
            let first = iterator.next().expect("Please provide a number mechuuu");
            let last = iterator.last().unwrap_or(first);
            let num = format!("{}{}",first,last).parse::<u32>().unwrap();
            num
        })
        .sum();
    println!("{sum}");
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(process("src/a10.txt").unwrap(), 56397.to_string());
    }
}
