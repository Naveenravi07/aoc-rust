use std::io;

pub fn process(input: &str) -> Result<u32, io::Error> {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let mut iterator = line.chars().filter_map(|character| character.to_digit(10));
            let first = iterator.next().expect("Please provide a number mechuuu");
            let last = iterator.last().unwrap_or(first);
            first*10+last
        })
        .sum();
    println!("{sum}");
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let file_content = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(process(file_content).unwrap(), 142);
    }
}
