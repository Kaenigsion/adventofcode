use std::fs;

fn main() {
    let path = "day-two/puzzle-input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");

    let choices = get_choices(content);
    let total_score = calculate_total_score(choices.clone());
    println!("The total score is {}", total_score);

    let total_win_score = calculate_total_win_score(choices);
    println!("The total win score is {}", total_win_score)
    // println!("{content}")
}

#[derive(Debug, Clone, Copy)]
enum Type {
    Rock,
    Paper,
    Scissor,
}

fn get_choices(content: String) -> [Vec<Type>; 2] {
    let mut opponent_choices: Vec<Type> = Vec::new();
    let mut my_choices: Vec<Type> = Vec::new();
    for line in content.lines() {
        let current_opponent_choice = line.chars().nth(0).unwrap();
        match current_opponent_choice {
            'A' => opponent_choices.push(Type::Rock),
            'B' => opponent_choices.push(Type::Paper),
            'C' => opponent_choices.push(Type::Scissor),
            _ => panic!(),
        }

        let current_my_choice = line.chars().nth(2).unwrap();
        match current_my_choice {
            'X' => my_choices.push(Type::Rock),
            'Y' => my_choices.push(Type::Paper),
            'Z' => my_choices.push(Type::Scissor),
            _ => panic!(),
        }
    }
    return [opponent_choices, my_choices];
}

fn calculate_total_score(choices: [Vec<Type>; 2]) -> usize {
    let [opponent_choices, my_choices] = choices;
    let mut total_score: usize = 0;

    for choices in opponent_choices.iter().zip(my_choices) {
        total_score += match choices {
            // Rock = 1; Paper = 2; Scissor = 3
            // loose = 0; draw = 3; win = 6
            (Type::Rock, Type::Rock) => 3 + 1,   // draw    + rock
            (Type::Rock, Type::Paper) => 6 + 2,  // win     + paper
            (Type::Rock, Type::Scissor) => 3,    // loose   + scissor
            (Type::Paper, Type::Rock) => 1,      // loose   + rock
            (Type::Paper, Type::Paper) => 3 + 2, // draw    + paper
            (Type::Paper, Type::Scissor) => 6 + 3, // win     + scissor
            (Type::Scissor, Type::Rock) => 6 + 1, // win     + rock
            (Type::Scissor, Type::Paper) => 2,   // loose   + paper
            (Type::Scissor, Type::Scissor) => 3 + 3, // draw    + scissor
        }
    }
    return total_score;
}

// PART 2---
fn calculate_total_win_score(choices: [Vec<Type>; 2]) -> usize {
    let [opponent_choices, my_choices] = choices;
    let mut new_my_choices: Vec<Type> = Vec::new();
    for current_choices in opponent_choices.iter().zip(my_choices) {
        new_my_choices.push(match current_choices {
            // rock (X) => loose; paper (Y) => draw; scissor (Z) => win
            (Type::Rock, Type::Rock) => Type::Scissor, // scissor looses against rock
            (Type::Rock, Type::Paper) => Type::Rock,   // rock draws against rock
            (Type::Rock, Type::Scissor) => Type::Paper, // paper wins against rock
            (Type::Paper, Type::Rock) => Type::Rock,   // ...
            (Type::Paper, Type::Paper) => Type::Paper,
            (Type::Paper, Type::Scissor) => Type::Scissor,
            (Type::Scissor, Type::Rock) => Type::Paper,
            (Type::Scissor, Type::Paper) => Type::Scissor,
            (Type::Scissor, Type::Scissor) => Type::Rock,
        });
    }
    let choices = [opponent_choices, new_my_choices];
    let total_win_score = calculate_total_score(choices);

    return total_win_score;
}
