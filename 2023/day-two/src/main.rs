use std::{fs, iter::Iterator};

fn main() {
    let path = "day-two/puzzle_input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");
    let sum1 = get_sum_of_game_ids(content.clone());
    let sum2 = denoise_and_get_sum2(content);

    println!("p1: {sum1}");
    println!("p2: {sum2}");
}

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn new(mut color_string: &str) -> Self {
        color_string = color_string.trim();
        let color = match color_string {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!(),
        };

        color
    }
}
#[derive(Debug)]
struct GameSet {
    quantity: usize,
    color: Color,
}

impl GameSet {
    fn new(cube_str: &str) -> Self {
        let (quantity_string, color_name) = cube_str.trim().split_once(' ').unwrap();
        let quantity = quantity_string.parse().unwrap();
        let color = Color::new(color_name);

        GameSet { quantity, color }
    }

    // p1

    fn matches_predicate(cubes: &Vec<Self>) -> bool {
        let cubes_red = GameSet::get_max_quantity(cubes, Color::Red);
        let cubes_blue = GameSet::get_max_quantity(cubes, Color::Blue);
        let cubes_green = GameSet::get_max_quantity(cubes, Color::Green);

        if cubes_red <= 12 && cubes_green <= 13 && cubes_blue <= 14 {
            return true;
        }

        false
    }

    fn get_max_quantity(cubes: &Vec<Self>, color: Color) -> usize {
        cubes
            .iter()
            .filter(|x| x.color == color)
            .map(|x| x.quantity)
            .max()
            .unwrap()
    }

    // p2

    fn get_power_of_mins(cubes: &Vec<Self>) -> usize {
        let cubes_red = GameSet::get_min_quantity(cubes, Color::Red);
        let cubes_blue = GameSet::get_min_quantity(cubes, Color::Blue);
        let cubes_green = GameSet::get_min_quantity(cubes, Color::Green);

        cubes_red * cubes_blue * cubes_green
    }

    fn get_min_quantity(cubes: &Vec<Self>, color: Color) -> usize {
        cubes
            .iter()
            .filter(|x| x.color == color)
            .map(|x| x.quantity)
            .max()
            .unwrap()
    }
}

// p1
fn get_sum_of_game_ids(content: String) -> usize {
    let mut matching_games = Vec::new();
    for line in content.lines() {
        let (game_name, rest) = line.split_once(':').unwrap();
        let game_id: usize = game_name
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap();
        let mut bag = Vec::new();

        for cube_set in rest.split(';') {
            for cube in cube_set.split(',') {
                let game_set = GameSet::new(cube);
                bag.push(game_set);
            }
        }

        let matches_predicate = GameSet::matches_predicate(&bag);
        if matches_predicate {
            matching_games.push(game_id)
        }
    }

    matching_games.iter().sum()
}

// p2
fn denoise_and_get_sum2(content: String) -> usize {
    let mut games = Vec::new();
    for line in content.lines() {
        let (_, rest) = line.split_once(':').unwrap();
        let mut bag = Vec::new();

        for cube_set in rest.split(';') {
            for cube in cube_set.split(',') {
                let game_set = GameSet::new(cube);
                bag.push(game_set);
            }
        }

        let power = GameSet::get_power_of_mins(&bag);
        games.push(power);
    }

    games.iter().sum()
}
