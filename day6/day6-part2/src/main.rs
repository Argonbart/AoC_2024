use array2d::Array2D;
use std::{collections::HashMap, usize};

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

    // Safe previous path
    let mut walkpath: Array2D<Vec<&str>> = Array2D::filled_with(
        Vec::new(),
        input.lines().into_iter().clone().next().unwrap().len(),
        input.lines().into_iter().count(),
    );

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

    let mut loop_counter = 0;
    let mut golem_active = true;
    // Check if next step would be a loop
    while golem_active {
        let next_field_pos = (
            (player_position.0 + player_direction.1) as usize,
            (player_position.1 + player_direction.2) as usize,
        );
        if is_valid_position(
            &gamefield,
            (next_field_pos.0 as i32, next_field_pos.1 as i32),
        ) {
            let next_field_symbols =
                &walkpath[((player_position.0) as usize, (player_position.1) as usize)];
            let mut numbers = Vec::new();
            if next_field_symbols.contains(&"0") {
                numbers.push(0);
            }
            if next_field_symbols.contains(&"1") {
                numbers.push(1);
            }
            if next_field_symbols.contains(&"2") {
                numbers.push(2);
            }
            if next_field_symbols.contains(&"3") {
                numbers.push(3);
            }
            for number in numbers {
                if (player_direction.0 + 1) % 4 == number {
                    loop_counter += 1;
                }
            }
        }

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
            match player_direction.0 {
                0 => {
                    walkpath = make_column_zero_below(
                        walkpath,
                        player_position.0 as usize,
                        player_position.1 as usize,
                    )
                }
                1 => {
                    walkpath = make_row_zero_left(
                        walkpath,
                        player_position.0 as usize,
                        player_position.1 as usize,
                    )
                }
                2 => {
                    walkpath = make_column_zero_above(
                        walkpath,
                        player_position.0 as usize,
                        player_position.1 as usize,
                    )
                }
                3 => {
                    walkpath = make_row_zero_right(
                        walkpath,
                        player_position.0 as usize,
                        player_position.1 as usize,
                    )
                }
                _ => println!("Invalid player directino input."),
            }
            player_position.0 += player_direction.1;
            player_position.1 += player_direction.2;
        }
    }

    // let mut counter = 0;
    // for col in 0..gamefield.column_len() {
    //     for row in 0..gamefield.row_len() {
    //         if gamefield[(col, row)] == "X" {
    //             counter += 1;
    //         }
    //     }
    // }

    println!("Result: {:?}", loop_counter)
}

fn is_valid_position(gamefield: &Array2D<&str>, player_position: (i32, i32)) -> bool {
    return player_position.0 >= 0
        && player_position.0 < gamefield.column_len() as i32
        && player_position.1 >= 0
        && player_position.1 < gamefield.row_len() as i32;
}

fn make_column_zero_below(
    walkpath: Array2D<Vec<&str>>,
    column: usize,
    row: usize,
) -> Array2D<Vec<&str>> {
    let mut return_walkpath = walkpath.clone();
    for c in 0..walkpath.column_len() {
        if column <= c {
            let mut temp_vec = return_walkpath[(c, row)].clone();
            if !temp_vec.contains(&"0") {
                temp_vec.push("0");
            }
            return_walkpath[(c, row)] = temp_vec;
        }
    }
    return return_walkpath;
}

fn make_column_zero_above(
    walkpath: Array2D<Vec<&str>>,
    column: usize,
    row: usize,
) -> Array2D<Vec<&str>> {
    let mut return_walkpath = walkpath.clone();
    for c in 0..walkpath.column_len() {
        if column >= c {
            let mut temp_vec = return_walkpath[(c, row)].clone();
            if !temp_vec.contains(&"2") {
                temp_vec.push("2");
            }

            return_walkpath[(c, row)] = temp_vec;
        }
    }
    return return_walkpath;
}

fn make_row_zero_left(
    walkpath: Array2D<Vec<&str>>,
    column: usize,
    row: usize,
) -> Array2D<Vec<&str>> {
    let mut return_walkpath = walkpath.clone();
    for r in 0..walkpath.row_len() {
        if row >= r {
            let mut temp_vec = return_walkpath[(column, r)].clone();
            if !temp_vec.contains(&"1") {
                temp_vec.push("1");
            }
            return_walkpath[(column, r)] = temp_vec;
        }
    }
    return return_walkpath;
}

fn make_row_zero_right(
    walkpath: Array2D<Vec<&str>>,
    column: usize,
    row: usize,
) -> Array2D<Vec<&str>> {
    let mut return_walkpath = walkpath.clone();
    for r in 0..walkpath.row_len() {
        if row <= r {
            let mut temp_vec = return_walkpath[(column, r)].clone();
            if !temp_vec.contains(&"3") {
                temp_vec.push("3");
            }
            return_walkpath[(column, r)] = temp_vec;
        }
    }
    return return_walkpath;
}

fn print_map(walkpath: &Array2D<Vec<&str>>) {
    println!("----");
    for column in 0..walkpath.column_len() {
        for row in 0..walkpath.row_len() {
            print!("{:?}", walkpath[(column, row)]);
        }
        println!("");
    }
    println!("----");
}
