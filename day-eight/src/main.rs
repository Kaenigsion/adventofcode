use std::fs;
fn main() {
    let path = "day-eight/puzzle-input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    let amount_visible_trees = get_amount_visible_trees(content);

    println!(
        "There are {} visible trees in the forest.",
        amount_visible_trees
    );
}

fn get_amount_visible_trees(content: String) -> usize {
    let mut visible_trees_amount = 0;
    // let mut scienic_scores: Vec<usize> = Vec::new(); // Part 2
    let amount_columns = content.lines().nth(0).unwrap().chars().count();
    let amount_rows = content.lines().count();
    visible_trees_amount += 2 * (amount_columns - 2);

    let mut forest_array: Vec<Vec<u8>> = vec![vec![0; amount_columns]; amount_rows];

    for colum in 0..amount_rows {
        visible_trees_amount += 2;
        for row in 0..amount_columns {
            let line = content.lines().nth(colum).unwrap();
            let current_value = line.chars().nth(row).unwrap().to_digit(10).unwrap();
            forest_array[colum][row] = current_value as u8;
        }
    }

    for tree_column_index in 1..(forest_array.iter().count() - 1) {
        for tree_row_index in 1..(amount_rows - 1) {
            let tree_height = forest_array[tree_column_index][tree_row_index];

            let mut top_trees: Vec<u8> = Vec::new();
            for i in 0..tree_column_index {
                top_trees.push(forest_array[i][tree_row_index]);
            }

            let mut bottom_trees: Vec<u8> = Vec::new();
            for i in (tree_column_index + 1)..amount_columns {
                bottom_trees.push(forest_array[i][tree_row_index]);
            }

            let mut left_trees: Vec<u8> = Vec::new();
            for i in 0..tree_row_index {
                left_trees.push(forest_array[tree_column_index][i]);
            }

            let mut right_trees: Vec<u8> = Vec::new();
            for i in (tree_row_index + 1)..amount_rows {
                right_trees.push(forest_array[tree_column_index][i]);
            }
            // // Part 2:
            // let mut is_bigger = false;
            // let mut multiplier = top_trees
            //     .iter()
            //     .filter(|&&x| {
            //         if x >= tree_height {
            //             is_bigger = true;
            //         }
            //         dbg!(x);
            //         return !is_bigger && x < tree_height;
            //     })
            //     .count();
            // dbg!(multiplier);
            // let mut is_bigger = false;
            // multiplier += bottom_trees
            //     .iter()
            //     .filter(|&&x| {
            //         if x >= tree_height {
            //             is_bigger = true;
            //         }
            //         dbg!(x);
            //         return is_bigger && x < tree_height;
            //     })
            //     .count();
            // dbg!(multiplier);
            // let mut is_bigger = false;
            // multiplier += left_trees
            //     .iter()
            //     .filter(|&&x| {
            //         if x >= tree_height {
            //             is_bigger = true;
            //         }
            //         dbg!(x);
            //         return is_bigger && x < tree_height;
            //     })
            //     .count();
            // dbg!(multiplier);
            // let mut is_bigger = false;
            // multiplier += right_trees
            //     .iter()
            //     .filter(|&&x| {
            //         if x >= tree_height {
            //             is_bigger = true;
            //         }
            //         dbg!(x);
            //         return is_bigger && x < tree_height;
            //     })
            //     .count();

            // dbg!(multiplier, tree_height);

            // Part 1:
            top_trees.sort();
            bottom_trees.sort();
            left_trees.sort();
            right_trees.sort();

            if tree_height > *top_trees.last().unwrap()
                || tree_height > *bottom_trees.last().unwrap()
                || tree_height > *left_trees.last().unwrap()
                || tree_height > *right_trees.last().unwrap()
            {
                visible_trees_amount += 1;
            }
        }
    }
    return visible_trees_amount;
}
