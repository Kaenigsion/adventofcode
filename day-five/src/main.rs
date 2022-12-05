use std::{collections::BTreeMap, fs};

use itertools::Itertools;

fn main() {
    let path = "day-five/puzzle-input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    let [value, value_part_two] = read_file_and_count_highest_stack(content);
    println!("Part 1: {}.", value);
    println!("Part 2: {}.", value_part_two);
}

fn read_file_and_count_highest_stack(content: String) -> [String; 2] {
    let mut stacks: BTreeMap<usize, Vec<char>> = BTreeMap::new();
    let mut break_index = 0;
    for (index, line) in content.lines().enumerate() {
        if line.is_empty() {
            break_index = index;
            break;
        }

        for (index, container) in line
            .chars()
            .enumerate()
            .filter(|(_, char)| char.is_alphabetic())
        {
            let index = index / 4 + 1;

            if !stacks.contains_key(&index) {
                stacks.insert(index, Vec::new());
            }

            stacks.get_mut(&index).unwrap().push(container);
        }
    }

    //part 2
    let mut stacks_part_two = stacks.clone();
    for index in break_index..content.lines().count() {
        let line = content.lines().nth(index).unwrap();

        for (count, start, end) in line
            .split(' ')
            .filter(|&char| char.parse::<usize>().is_ok())
            .tuples()
        {
            let count = count.parse::<usize>().unwrap();
            let start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();

            let stack = stacks.get_mut(&start).unwrap();
            let containers = stack.drain(..count).rev().collect::<Vec<char>>();

            let previous_containers = stacks.get(&end).unwrap().clone();
            let new_containers = [containers, previous_containers.clone()].concat();

            stacks.insert(end, new_containers);

            // part 2
            let stack = stacks_part_two.get_mut(&start).unwrap();
            let containers_part_two = stack.drain(..count).collect::<Vec<char>>();

            let previous_containers = stacks_part_two.get(&end).unwrap().clone();
            let new_containers_part_two = [containers_part_two, previous_containers].concat();

            stacks_part_two.insert(end, new_containers_part_two);
        }
    }

    let mut top_stack = Vec::new();
    for value in stacks.values() {
        top_stack.push(*value.first().unwrap())
    }

    // part 2
    let mut top_stack_part_two = Vec::new();
    for value in stacks_part_two.values() {
        top_stack_part_two.push(*value.first().unwrap())
    }
    return [
        top_stack.iter().collect(),
        top_stack_part_two.iter().collect(),
    ];
}
