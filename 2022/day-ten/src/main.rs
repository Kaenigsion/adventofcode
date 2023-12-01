use std::fs;
fn main() {
    let path = "day-ten/puzzle-input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    let part1 = get_signal_strengths(content);
    println!("The sum of the six signal strengths is {part1}.")
}

fn get_signal_strengths(content: String) -> i32 {
    let mut tick_amount: u16 = 1;
    let mut register_value: i32 = 1;
    let cycle_stops: [u16; 6] = [20, 60, 100, 140, 180, 220];

    let mut result = Vec::new();

    content.lines().for_each(|line| {
        if line.starts_with("addx ") {
            tick_amount += 1;
            if cycle_stops.contains(&tick_amount) {
                result.push(register_value);
            }

            register_value += line.split(' ').next_back().unwrap().parse::<i32>().unwrap();
            tick_amount += 1;
        } else {
            tick_amount += 1
        }

        if cycle_stops.contains(&tick_amount) {
            result.push(register_value)
        }
    });

    // Part 2

    // Part 1
    let sum = result
        .iter()
        .enumerate()
        .map(|(index, value)| value * cycle_stops[index] as i32)
        .sum::<i32>();

    return sum;
}
