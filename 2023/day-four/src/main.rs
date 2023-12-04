use std::fs;

fn main() {
    let path = "day-four/puzzle_input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    let sum1 = get_sum_of_winning_numbers(content.clone());
    // let sum2 = get_sum_of_power_mins(content);

    println!("p1: {sum1}");
    // println!("p2: {sum2}");
}

// p1
fn get_sum_of_winning_numbers(content: String) -> usize {
    let mut winning_number_sum: usize = 0;
    for line in content.lines() {
        let mut amount: u32 = 0;
        let (_, rest) = line.split_once(':').unwrap();
        let (my_numbers, winning_numbers) = rest.split_once('|').unwrap();

        let my_numbers: Vec<&str> = my_numbers
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect();
        let winning_numbers: Vec<&str> = winning_numbers
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect();

        my_numbers.iter().for_each(|x| {
            if winning_numbers.contains(&x.trim().to_string().as_str()) {
                amount += 1;
            }
        });

        if amount != 0 {
            winning_number_sum += 2_usize.pow(amount - 1);
        }
    }
    winning_number_sum
}
