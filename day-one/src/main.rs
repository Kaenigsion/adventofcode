use std::{fs, iter::Iterator};

fn main() {
    let path = "day-one/puzzle-input.txt";
    let content = fs::read_to_string(path).expect("couldnt open file");
    let elves = get_elves(content);
    let [most_calories, sum_three_most_calories] = get_elf_with_most_calories(elves);
    println!("The elf carrying the most calories is {}.", most_calories);
    println!(
        "The sum of the calories from the three most carrying elves is {}.",
        sum_three_most_calories
    );
}

fn get_elves(content: String) -> Vec<Vec<usize>> {
    let mut elves: Vec<Vec<usize>> = Vec::new();
    elves.push(Vec::new());
    // get elves out of context
    let mut index: usize = 0;
    for food_item in content.split('\n') {
        if food_item == "" {
            elves.push(Vec::new());
            index += 1;
            continue;
        }
        let elf = elves.get_mut(index).unwrap();
        elf.push(food_item.parse().expect("Wrong syntax."));
    }

    return elves;
}

fn get_elf_with_most_calories(elves: Vec<Vec<usize>>) -> [usize; 2] {
    let mut elf_with_max_calories: Vec<usize> = Vec::new();
    for elf in elves.iter() {
        let total_calories: usize = elf.iter().sum(); // sum up
        elf_with_max_calories.push(total_calories);
    }
    elf_with_max_calories.sort();
    let last_three_elves_total_calories = elf_with_max_calories.iter().rev().take(3).sum(); // take last 3 and sum
    let elf_with_max_calories = *elf_with_max_calories.last().unwrap(); // get biggest item
    return [elf_with_max_calories, last_three_elves_total_calories];
}
