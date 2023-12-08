use std::{fs, io};

const NUM_SPELLED: [(&str, &str); 10] = [
    ("one", "one1one"),
    ("two", "two2two"),
    ("three", "three3three"),
    ("four", "four4four"),
    ("five", "five5five"),
    ("six", "six6six"),
    ("seven", "seven7seven"),
    ("eight", "eight8eight"),
    ("nine", "nine9nine"),
    ("zero", "zero0zero"),
];

pub fn process(input: &str) -> Result<u32, io::Error> {
    let file_content: String = fs::read_to_string(input)?;

    let my_new_brain_is_ozm = file_content
        .lines()
        .map(|line| {
            let updated_line =
                NUM_SPELLED.map(|(num_str, num_spelled)| line.replace(num_str, num_spelled));
            updated_line.last().unwrap().to_string()
        })
    .map(|updated_lines_iter|{
        updated_lines_iter
            .chars()
            .filter_map(|c|c.to_digit(10))
            .collect::<Vec<u32>>()
    })
    .map(|vec|10*vec.first().unwrap()+vec.last().unwrap())
        .sum();
    println!("{my_new_brain_is_ozm}");
    Ok(my_new_brain_is_ozm)
}

fn main() {
    process("d1p2.txt").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(process("src/a10.txt").unwrap(), 56397.to_string());
    }
}

