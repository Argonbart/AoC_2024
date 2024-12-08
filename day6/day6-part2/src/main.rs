use array2d::Array2D;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    // Start Timer
    let timing_start = Instant::now();

    // Read Input
    let input = include_str!("../input.txt");

    // Define game variables
    let mut gamefield = Array2D::filled_with(
        ".",
        input.lines().into_iter().clone().next().unwrap().len(),
        input.lines().into_iter().count(),
    );
    let mut player_position: (usize, usize) = (0, 0);
    let mut player_direction: i32 = 0;
    let mut direction_dict: HashMap<i32, (i32, i32)> = HashMap::new();
    direction_dict.insert(0, (-1, 0));
    direction_dict.insert(1, (0, 1));
    direction_dict.insert(2, (1, 0));
    direction_dict.insert(3, (0, -1));

    // Read input into gamefield and player variables
    for line_pos in 0..input.lines().count() {
        for char_pos in 0..input.lines().nth(0).unwrap().len() {
            let current_character = input
                .lines()
                .nth(line_pos)
                .unwrap()
                .chars()
                .nth(char_pos)
                .unwrap();
            if current_character == '.' {
                continue;
            } else if current_character == '#' {
                gamefield[(line_pos, char_pos)] = "#";
            } else {
                player_position = (line_pos, char_pos);
                match current_character {
                    '^' => {
                        player_direction = 0;
                    }
                    '>' => {
                        player_direction = 1;
                    }
                    'v' => {
                        player_direction = 2;
                    }
                    '<' => {
                        player_direction = 3;
                    }
                    _ => {
                        println!("Invalid character input.");
                    }
                }
            }
        }
    }

    // try placing obstacle at any position and see if loops appears
    let mut counter = 0;
    for column in 0..gamefield.column_len() {
        for row in 0..gamefield.row_len() {
            let mut visisted_positions: Vec<(usize, usize, i32)> = Vec::new();
            let mut found_loop = false;
            let mut golem_active = true;

            // Clone new setup
            let mut gamefield = gamefield.clone();
            let mut player_position = player_position.clone();
            let mut player_direction = player_direction.clone();
            let direction_dict = direction_dict.clone();

            // Set obstacle
            gamefield[(column, row)] = "#";

            // Move golem
            while golem_active {
                let follow_position_i32 = (
                    (player_position.0 as i32 + direction_dict.get(&player_direction).unwrap().0),
                    (player_position.1 as i32 + direction_dict.get(&player_direction).unwrap().1),
                );
                let follow_position = (
                    (player_position.0 as i32 + direction_dict.get(&player_direction).unwrap().0)
                        as usize,
                    (player_position.1 as i32 + direction_dict.get(&player_direction).unwrap().1)
                        as usize,
                );

                // golem out of bounds = end
                if !(follow_position_i32.0 >= 0
                    && follow_position_i32.0 < gamefield.column_len() as i32
                    && follow_position_i32.1 >= 0
                    && follow_position_i32.1 < gamefield.row_len() as i32)
                {
                    gamefield[(player_position.0 as usize, player_position.1 as usize)] = "X";
                    golem_active = false;
                    continue;
                }
                // something in the way
                else if gamefield[follow_position] == "#" {
                    // turn player 90° cw
                    let rotated_direction = (player_direction + 1) % 4;
                    player_direction = rotated_direction;
                }
                // free path forwards
                else {
                    // make a step
                    if visisted_positions.contains(&(
                        player_position.0 as usize,
                        player_position.1 as usize,
                        player_direction,
                    )) {
                        found_loop = true;
                        golem_active = false;
                        continue;
                    }

                    gamefield[(player_position.0 as usize, player_position.1 as usize)] = "X";
                    visisted_positions.push((
                        player_position.0 as usize,
                        player_position.1 as usize,
                        player_direction,
                    ));
                    player_position = follow_position;
                }
            }

            if found_loop {
                counter += 1;
            }

            gamefield[(column, row)] = ".";
        }
    }

    // Output Result
    let duration = timing_start.elapsed();
    println!("Result: {:?}", counter);
    println!("Time elapsed: {:?}", duration);
}
