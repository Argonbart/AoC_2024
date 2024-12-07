use std::collections::HashMap;

use array2d::Array2D;

fn main() {
    // Read Input
    let input = include_str!("../input.txt");

    // Define game variables
    let mut gamefield = Array2D::filled_with(
        ".",
        input.lines().into_iter().clone().next().unwrap().len(),
        input.lines().into_iter().count(),
    );
    let mut player_position: (i32, i32) = (0, 0);
    let mut player_direction: (i32, i32, i32) = (0, 0, 0);
    let mut direction_dict: HashMap<i32, (i32, i32)> = HashMap::new();
    // up    = 0 = (-1, 0) = (0, -1, 0)
    direction_dict.insert(0, (-1, 0));
    // right = 1 = ( 0, 1) = (1, 0, 1)
    direction_dict.insert(1, (0, 1));
    // down  = 2 = ( 1, 0) = (2, 1, 0)
    direction_dict.insert(2, (1, 0));
    // left  = 3 = ( 0,-1) = (3, 0, -1)
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
                player_position = (line_pos as i32, char_pos as i32);
                match current_character {
                    '^' => {
                        player_direction = (0, -1, 0);
                    }
                    '>' => {
                        player_direction = (1, 0, 1);
                    }
                    'v' => {
                        player_direction = (2, 1, 0);
                    }
                    '<' => {
                        player_direction = (3, 0, -1);
                    }
                    _ => {
                        println!("Invalid character input.");
                    }
                }
            }
        }
    }

    let mut golem_active = true;
    while golem_active {
        // golem out of bounds = end
        if !is_valid_position(
            &gamefield,
            (
                (player_position.0 + player_direction.1),
                (player_position.1 + player_direction.2),
            ),
        ) {
            gamefield[(player_position.0 as usize, player_position.1 as usize)] = "X";
            golem_active = false;
            continue;
        }
        // something in the way
        else if gamefield[(
            (player_position.0 + player_direction.1) as usize,
            (player_position.1 + player_direction.2) as usize,
        )] == "#"
        {
            // turn player 90° cw
            let rotated_direction = (player_direction.0 + 1) % 4;
            player_direction = (
                rotated_direction,
                direction_dict.get(&rotated_direction).unwrap().0,
                direction_dict.get(&rotated_direction).unwrap().1,
            );
        }
        // free path forwards
        else {
            // make a step
            gamefield[(player_position.0 as usize, player_position.1 as usize)] = "X";
            player_position.0 += player_direction.1;
            player_position.1 += player_direction.2;
        }
    }

    let mut counter = 0;
    for col in 0..gamefield.column_len() {
        for row in 0..gamefield.row_len() {
            if gamefield[(col, row)] == "X" {
                counter += 1;
            }
        }
    }

    println!("{:?}", counter)
}

fn is_valid_position(gamefield: &Array2D<&str>, player_position: (i32, i32)) -> bool {
    return player_position.0 >= 0
        && player_position.0 < gamefield.column_len() as i32
        && player_position.1 >= 0
        && player_position.1 < gamefield.row_len() as i32;
}
