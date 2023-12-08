use std::{fs, io, num, u32};

pub fn process(input: &str) -> Result<String, io::Error> {
    let file_content: String = fs::read_to_string(input)?;

    let sum: u32 = file_content
        .lines()
        .map(|line| {
            let mut iterator = line.chars();
            let first = iterator.next().unwrap().to_digit(10);
            let last = iterator.last().unwrap().to_digit(10);
            
            type numbertype = (&str,u32);
            let numbers : [numbertype]  = [
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7), 
                ("eight", 8),
                ("nine", 9),
                ("zero", 0),
            ];

            let num_1 : u32 = match first {
                Some(num)=>num,
                None => {
                    let p1 = &[0,line.len()/2];
                    numbers
                        .iter()
                        .filter_map(|(i,n)|line.find(i))
                        .collect()
                }
            };

            let num_2 : u32 = match last {
                Some(num) => num,
                None => {
                    let p1 = &[line.len()/2,..];
                    numbers
                        .iter()
                        .filter_map(|(i,n)|line.find(i))
                        .collect()


                }
            };
            let num = format!("{}{}", num_1, num_2);
            println!("{num}");
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
