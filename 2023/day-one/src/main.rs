use std::{fs, iter::Iterator};

fn main() {
    let path = "day-one/puzzle_input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    let sum1 = denoise_and_get_sum1(content.clone());
    // let sum2 = denoise_and_get_sum2(content);

    println!("p1: {sum1}");
    // println!("p2: {sum2}");
}

// p1
fn denoise_and_get_sum1(content: String) -> usize {
    let mut values = Vec::new();
    for line in content.lines() {
        let all_numbers: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();
        let first = all_numbers.first().unwrap();
        let last = all_numbers.last().unwrap();

        values.push(format!("{first}{last}"))
    }

    let sum = values.iter().map(|x| x.parse::<usize>().unwrap()).sum();

    return sum;
}
