use std::fs;

fn main() {
    let path = "day-six/puzzle_input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    let location1 = get_best_races(content.clone());
    println!("p1: {location1}");

    let location2 = get_best_races2(content);
    println!("p2: {location2}");
}

fn get_numbers(content: &str) -> Vec<usize> {
    content
        .split(' ')
        .filter(|x| x.trim().chars().all(char::is_numeric) && !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

// p1
fn get_best_races(content: String) -> usize {
    let (times, distances) = content.split_once('\n').unwrap();
    let times = get_numbers(times);
    let distances = get_numbers(distances);

    let mut races = 1;

    for (time, distance) in times.iter().zip(distances) {
        let mut better_races = 0;
        for i in 0..*time {
            if i * (time - i) > distance {
                better_races += 1;
            }
        }
        races *= better_races;
    }
    races
}

// // p2
// p1
fn get_best_races2(content: String) -> usize {
    let (times, distances) = content.split_once('\n').unwrap();
    let time: usize = get_numbers(times)
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: usize = get_numbers(distances)
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse()
        .unwrap();

    let mut races = 1;

    let mut better_races = 0;
    for i in 0..time {
        if i * (time - i) > distance {
            better_races += 1;
        }
    }
    races *= better_races;

    races
}
