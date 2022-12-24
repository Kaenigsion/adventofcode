use std::fs;

fn main() {
    let path = "day-twentyfour/puzzle-input.txt";
    let content = fs::read_to_string(path).expect("couldn't open file");

    get_shortest_path(content);
}

#[derive(Debug, Clone, Copy)]
struct Player {
    position_x: usize,
    position_y: usize,
}
impl Player {
    pub fn new([position_x, position_y]: [usize; 2]) -> Self {
        Self {
            position_x,
            position_y,
        }
    }
}
#[derive(Debug)]
struct Blizzard {
    pos_x: usize,
    pos_y: usize,
    direction: u8, // 0: north, 1: east, 2: south, 3: west
}

fn get_shortest_path(content: String) -> u32 {
    let mut field: Vec<Vec<char>> = Vec::new();
    let mut blizzards: Vec<Blizzard> = Vec::new();

    for (pos_y, line) in content.lines().enumerate() {
        for (pos_x, char) in line.chars().enumerate() {
            match char {
                '^' => blizzards.push(Blizzard {
                    pos_x,
                    pos_y,
                    direction: 0,
                }),
                '>' => blizzards.push(Blizzard {
                    pos_x,
                    pos_y,
                    direction: 1,
                }),
                'v' => blizzards.push(Blizzard {
                    pos_x,
                    pos_y,
                    direction: 2,
                }),
                '<' => blizzards.push(Blizzard {
                    pos_x,
                    pos_y,
                    direction: 3,
                }),
                _ => (),
            }
        }
        field.push(Vec::from_iter(line.chars()))
    }

    dbg!(&blizzards);

    let length_x = field[0].len() - 1;
    let length_y = field.len() - 1;
    dbg!(length_x, length_y);
    let mut players: Vec<Player> = Vec::new();
    players.push(Player::new([1, 0]));
    let mut minute: u32 = 0;
    let mut is_finished = false;
    // while !is_finished {
    for _ in 0..3 {
        minute += 1;
        // update Blizzards
        for blizzard in blizzards.iter_mut() {
            dbg!(&blizzard);
            field[blizzard.pos_y][blizzard.pos_x] = '.';
            match blizzard.direction {
                0 => {
                    blizzard.pos_y -= 1;
                    if blizzard.pos_y == 0 {
                        blizzard.pos_y = length_y - 1; // I assume here that there is no blizzard going upward/downward in the last field
                    }
                }
                1 => {
                    blizzard.pos_x += 1;
                    if blizzard.pos_x == length_x {
                        blizzard.pos_x = 1;
                    }
                }
                2 => {
                    blizzard.pos_y += 1;
                    if blizzard.pos_y == length_y {
                        blizzard.pos_y = 1; // I assume here that there is no blizzard going upward/downward in the first field
                    }
                }
                3 => {
                    blizzard.pos_x -= 1;
                    if blizzard.pos_x == 0 {
                        blizzard.pos_x = length_x - 1;
                    }
                }
                _ => panic!(),
            }
            field[blizzard.pos_y][blizzard.pos_x] = '.';
            dbg!(&blizzard);
        }
        for blizzard in blizzards.iter() {
            field[blizzard.pos_y][blizzard.pos_x] = blizzard.direction as char;
        }
        dbg!(&field);

        // player_logic
        for player in players.clone().iter() {
            if player.position_x == length_x - 1 && player.position_y == length_y {
                is_finished = true;
                println!("GG: {}", minute);
                continue;
            }

            let has_surrounding_left = field[player.position_y][player.position_x - 1] == '.';
            let has_surrounding_right = field[player.position_y][player.position_x + 1] == '.';
            let has_surrounding_top = if player.position_y == 0 {
                false
            } else {
                field[player.position_y - 1][player.position_x] == '.'
            };
            let has_surrounding_bottom = field[player.position_y + 1][player.position_x] == '.';

            let mut last_player_index: usize = 0;

            let all_surroundings = [
                has_surrounding_top,
                has_surrounding_right,
                has_surrounding_bottom,
                has_surrounding_left,
            ];
            all_surroundings
                .iter()
                .enumerate()
                .filter(|(_, &is_valid)| is_valid)
                .for_each(|(index, _)| {
                    players.remove(last_player_index);
                    match index {
                        0 => players.push(Player::new([player.position_x, player.position_y - 1])),
                        1 => players.push(Player::new([player.position_x + 1, player.position_y])),
                        2 => players.push(Player::new([player.position_x, player.position_y + 1])),
                        3 => players.push(Player::new([player.position_x - 1, player.position_y])),
                        _ => panic!(),
                    };
                    println!("index: {}", index)
                });

            last_player_index += all_surroundings
                .iter()
                .enumerate()
                .filter(|(_, &is_valid)| is_valid)
                .count();
            dbg!(
                has_surrounding_top,
                has_surrounding_right,
                has_surrounding_bottom,
                has_surrounding_left
            );
        }
        dbg!(&players);

        dbg!(field[players[0].position_y][players[0].position_x]);

        dbg!(minute);
    }

    dbg!(field);
    minute
}
