use std::io;

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
    let my_new_brain_is_ozm = input
        .lines()
        .map(|line| {
            let updated_line = NUM_SPELLED.iter().fold(line.to_string(), |acc, (num_str, num_spelled)| {
                acc.replace(num_str, num_spelled)
            });
            updated_line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| {
            return 10 * vec.first().unwrap_or(&0) + vec.last().unwrap_or(&0);
        })
        .sum();

    println!("{}", my_new_brain_is_ozm);
    Ok(my_new_brain_is_ozm)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let file_cnt = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(process(file_cnt).unwrap(), 281);
    }
}
