use std::fs;

fn main() {
    let path = "day-four/puzzle-input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    let [part_one, part_two] = get_amount_elf_fully_containing_other(content);
    println!("There are {part_one} fully common pairs.");
    println!("There are {part_two} common pairs.");
}

fn get_amount_elf_fully_containing_other(content: String) -> [u32; 2] {
    let mut amount_common_pairs: u32 = 0;
    let mut amount_common_pairs_two: u32 = 0;
    for elf_pair in content.lines() {
        let elf_pair = elf_pair.trim();
        let elf_pair: [&str; 2] = elf_pair
            .split(',')
            .collect::<Vec<&str>>()
            .try_into()
            .expect("wrong input");

        let elf_pair_numbers: [[usize; 2]; 2] = elf_pair.map(|elf| {
            return elf
                .split('-')
                .collect::<Vec<&str>>()
                .iter()
                .map(|&elf_item| elf_item.parse().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();
        });

        if elf_pair_numbers[0][1] <= elf_pair_numbers[1][1]
            && elf_pair_numbers[0][0] >= elf_pair_numbers[1][0]
            || elf_pair_numbers[0][1] >= elf_pair_numbers[1][1]
                && elf_pair_numbers[0][0] <= elf_pair_numbers[1][0]
        {
            amount_common_pairs += 1;
        }

        let elf_one =
            (elf_pair_numbers[0][0]..(elf_pair_numbers[0][1] + 1)).collect::<Vec<usize>>();
        let elf_two =
            (elf_pair_numbers[1][0]..(elf_pair_numbers[1][1] + 1)).collect::<Vec<usize>>();

        'for_break: for number in elf_one {
            for number_two in &elf_two {
                if number == *number_two {
                    amount_common_pairs_two += 1;
                    break 'for_break;
                }
            }
        }
    }
    [amount_common_pairs, amount_common_pairs_two]
}
