use std::fs;


fn main(){
    let mut  file_content = fs::read("d2p2.txt").unwrap();
    let result = day02::part2::process(&mut  file_content).unwrap();
    println!("Answer of day2 part 1 is : {}",result);
}

