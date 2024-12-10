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
                sum += find_hiking_paths(&hiking_map, (line, pos), 0);
            }
        }
    }

    // Output Result
    println!("Result: {:?}", sum);
}

fn find_hiking_paths(hiking_map: &Array2D<i32>, position: (usize, usize), number: i32) -> i32 {
    // Check if top reached
    if number == 9 {
        return 1;
    }
    // Check up
    let mut upper_value = 0;
    let upper_position = (position.0 - 1, position.1);
    if position.0 > 0 && hiking_map[upper_position] == number + 1 {
        upper_value = find_hiking_paths(hiking_map, upper_position, number + 1);
    }
    // Check down
    let mut lower_value = 0;
    let lower_position = (position.0 + 1, position.1);
    if position.0 < hiking_map.column_len() - 1 && hiking_map[lower_position] == number + 1 {
        lower_value = find_hiking_paths(hiking_map, lower_position, number + 1);
    }
    // Check left
    let mut left_value = 0;
    let left_position = (position.0, position.1 - 1);
    if position.1 > 0 && hiking_map[left_position] == number + 1 {
        left_value = find_hiking_paths(hiking_map, left_position, number + 1);
    }
    // Check right
    let mut right_value = 0;
    let right_position = (position.0, position.1 + 1);
    if position.1 < hiking_map.row_len() - 1 && hiking_map[right_position] == number + 1 {
        right_value = find_hiking_paths(hiking_map, right_position, number + 1);
    }
    // Add up values
    return upper_value + lower_value + left_value + right_value;
}
