use std::collections::HashSet;

use array2d::Array2D;

fn main() {
    // Read Input
    let input = include_str!("../input.txt");

    // Define Variables
    let mut hiking_map: Array2D<i32> = Array2D::filled_with(
        0,
        input.lines().into_iter().clone().next().unwrap().len(),
        input.lines().into_iter().count(),
    );

    // Read input into hiking map
    const RADIX: u32 = 10;
    for line in 0..input.lines().count() {
        for pos in 0..input.lines().nth(0).unwrap().len() {
            hiking_map[(line, pos)] = input
                .lines()
                .nth(line)
                .unwrap()
                .chars()
                .nth(pos)
                .unwrap()
                .to_digit(RADIX)
                .unwrap() as i32;
        }
    }

    // Search for 0's to start at
    let mut sum = 0;
    for line in 0..hiking_map.column_len() {
        for pos in 0..hiking_map.row_len() {
            if hiking_map[(line, pos)] == 0 {
                // Count and add all found hiking paths
                sum += find_hiking_paths(&hiking_map, (line, pos), 0)
                    .iter()
                    .count();
            }
        }
    }

    // Output Result
    println!("Result: {:?}", sum);
}

fn find_hiking_paths(
    hiking_map: &Array2D<i32>,
    position: (usize, usize),
    number: i32,
) -> Vec<(usize, usize)> {
    let mut return_vector = Vec::new();
    // Check if top reached
    if number == 9 {
        return_vector.push(position);
        return return_vector;
    }
    // Check up
    let mut upper_value = Vec::new();
    let upper_position = (position.0 - 1, position.1);
    if position.0 > 0 && hiking_map[upper_position] == number + 1 {
        upper_value = find_hiking_paths(hiking_map, upper_position, number + 1);
    }
    // Check down
    let mut lower_value = Vec::new();
    let lower_position = (position.0 + 1, position.1);
    if position.0 < hiking_map.column_len() - 1 && hiking_map[lower_position] == number + 1 {
        lower_value = find_hiking_paths(hiking_map, lower_position, number + 1);
    }
    // Check left
    let mut left_value = Vec::new();
    let left_position = (position.0, position.1 - 1);
    if position.1 > 0 && hiking_map[left_position] == number + 1 {
        left_value = find_hiking_paths(hiking_map, left_position, number + 1);
    }
    // Check right
    let mut right_value = Vec::new();
    let right_position = (position.0, position.1 + 1);
    if position.1 < hiking_map.row_len() - 1 && hiking_map[right_position] == number + 1 {
        right_value = find_hiking_paths(hiking_map, right_position, number + 1);
    }
    // Add all directions up
    return_vector.append(&mut upper_value);
    return_vector.append(&mut lower_value);
    return_vector.append(&mut left_value);
    return_vector.append(&mut right_value);
    // Remove duplicates
    let set: HashSet<_> = return_vector.into_iter().collect();
    return_vector = set.into_iter().collect();
    return return_vector;
}
