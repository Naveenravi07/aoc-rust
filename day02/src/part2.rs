use std::io;

pub fn process(input: &mut Vec<u8>) -> Result<usize, io::Error> {
    input.remove(input.len()-1);
     let answer =input
         .split(|b| b == &b'\n')
            .enumerate()
            .filter_map(|(_, line)| {
                let mut rgb = [0, 0, 0];
                line.splitn(2, |b| b == &b':')
                    .nth(1)
                    .unwrap()
                    .split(|b| b == &b',' || b == &b';')
                    .all(|item| {
                        let mut set = item[1..].splitn(2, |b| *b == b' '); 
                        let qnty = set.nth(0).unwrap();
                        let colour = set.nth(0).unwrap();

                        let i = match colour{
                            b"red" => 0usize,
                            b"green" => 1,
                            b"blue" => 2,
                            _ => 0, 
                        };
                        rgb[i] = rgb[i].max(atoi::atoi(qnty).unwrap());
                        true
                    })
                    .then_some(rgb[0]*rgb[1]*rgb[2])
            })
            .sum::<usize>();
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let content = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(process(&mut content.as_bytes().to_vec()).unwrap(), 2286);
    }
}

