use itertools::Itertools;
use std::fs;

fn main() {
    let path = "day-three/puzzle-input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    let items = get_rucksack_compartments(content.clone());
    let sum = get_sum(items);
    println!("The total sum of the wrong compartments is {sum}.");

    let common_items_sum = get_group_badges(content);
    println!(
        "The total sum of the group badges compartments is {}.",
        common_items_sum
    );
}

fn get_rucksack_compartments(content: String) -> Vec<char> {
    let mut rucksacks: Vec<[&str; 2]> = Vec::new();
    for rucksack in content.lines() {
        let compartment_split_index = rucksack.len() / 2;
        let first_compartment = &rucksack[..compartment_split_index];
        let second_compartment = &rucksack[compartment_split_index..];

        rucksacks.push([first_compartment, second_compartment]);
    }

    let mut items: Vec<char> = Vec::new();
    for &[first_compartment, second_compartment] in rucksacks.iter() {
        'break_loop: for first_item in first_compartment.chars() {
            for second_item in second_compartment.chars() {
                if first_item == second_item {
                    items.push(first_item);
                    break 'break_loop; // in every rucksack there is just one item wrong
                }
            }
        }
    }

    return items;
}

fn get_sum(items: Vec<char>) -> usize {
    return items
        .iter()
        .map(|&character| alphabet_to_number(character))
        .sum();
}

fn alphabet_to_number(character: char) -> usize {
    let number = (character as u8)
        - match character.is_lowercase() {
            true => 96,  // letter 'a' is 97th on ascii table
            false => 38, // letter 'A' is 65th on ascii table
        };
    return number as usize;
}

// PART 2
fn get_group_badges(content: String) -> usize {
    let mut group_rucksacks: Vec<[&str; 3]> = Vec::new();
    for (first_elv, second_elf, third_elf) in content.lines().tuples() {
        group_rucksacks.push([first_elv, second_elf, third_elf])
    }

    let mut common_item: Vec<char> = Vec::new();
    for &[first_elf, second_elf, third_elf] in group_rucksacks.iter() {
        'break_loop: for first_item in first_elf.chars() {
            for second_item in second_elf.chars() {
                for third_item in third_elf.chars() {
                    if first_item == second_item && second_item == third_item {
                        common_item.push(first_item);
                        break 'break_loop; // in every rucksack there is just one item wrong
                    }
                }
            }
        }
    }
    return get_sum(common_item);
}
