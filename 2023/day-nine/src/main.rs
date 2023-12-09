use itertools::Itertools;
use std::fs;

fn main() {
    let path = "day-nine/puzzle_input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    let sum1 = sum_extrapolated_values1(content.clone());
    println!("p1: {sum1}");
    let sum2 = sum_extrapolated_values2(content.clone());
    println!("p1: {sum2}");
}

// p1
fn sum_extrapolated_values1(content: String) -> isize {
    let mut sum = 0;
    for line in content.lines() {
        let values: Vec<isize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();
        let mut extrapolated_values = vec![values];

        loop {
            let last_vec = extrapolated_values.last().unwrap().iter();
            let new_extrapolated_values: Vec<isize> = last_vec
                .tuple_windows()
                .map(|(first, second)| second - first)
                .collect();

            if new_extrapolated_values.iter().all(|x| x == &0) {
                break;
            }
            extrapolated_values.push(new_extrapolated_values);
        }

        let sth = extrapolated_values
            .iter()
            .map(|x| *x.last().unwrap())
            .reduce(|acc, e| acc + e)
            .unwrap();

        sum += sth;
    }
    sum
}

// p2
fn sum_extrapolated_values2(content: String) -> isize {
    let mut sum = 0;
    for line in content.lines() {
        let values: Vec<isize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();
        let mut extrapolated_values = vec![values];

        loop {
            let last_vec = extrapolated_values.last().unwrap().iter();
            let new_extrapolated_values: Vec<isize> = last_vec
                .tuple_windows()
                .map(|(first, second)| second - first)
                .collect();

            if new_extrapolated_values.iter().all(|x| x == &0) {
                break;
            }
            extrapolated_values.push(new_extrapolated_values);
        }

        let sth = extrapolated_values
            .iter()
            .map(|x| *x.first().unwrap())
            .rev()
            .reduce(|acc, e| e - acc)
            .unwrap();

        sum += sth;
    }
    sum
}
