use std::str::Chars;

fn main() {
    // Read Input
    let input = include_str!("../input.txt");

    // Define variables
    let field: Vec<Chars<'_>> = input.lines().map(|x| x.chars()).collect();
    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    // Read all available symbols
    let mut symbols: Vec<char> = Vec::new();
    for line in input.lines() {
        for symbol in line.chars() {
            if !symbols.contains(&symbol) && symbol != '.' {
                symbols.push(symbol);
            }
        }
    }

    // For every symbol
    for symbol in symbols {
        let mut symbol_appearances: Vec<(i32, i32)> = Vec::new();
        // Find all appearances
        for column in 0..input.lines().count() {
            for row in 0..input.lines().nth(0).unwrap().len() {
                if input.lines().nth(column).unwrap().chars().nth(row).unwrap() == symbol {
                    symbol_appearances.push((column as i32, row as i32));
                }
            }
        }

        // Go through each pairing
        for item_pos in 0..symbol_appearances.len() {
            for pos in 0..symbol_appearances.len() {
                // paired with itself
                if item_pos == pos {
                    continue;
                }

                // positions of pair
                let point_a_position = symbol_appearances.get(item_pos).unwrap();
                let point_b_position = symbol_appearances.get(pos).unwrap();

                // vectors
                let a_to_b_vector = (
                    point_b_position.0 - point_a_position.0,
                    point_b_position.1 - point_a_position.1,
                );
                let b_to_a_vector = (
                    point_a_position.0 - point_b_position.0,
                    point_a_position.1 - point_b_position.1,
                );

                // new antinodes
                let antinode1 = (
                    point_a_position.0 + 2 * a_to_b_vector.0,
                    point_a_position.1 + 2 * a_to_b_vector.1,
                );
                let antinode2 = (
                    point_b_position.0 + 2 * b_to_a_vector.0,
                    point_b_position.1 + 2 * b_to_a_vector.1,
                );

                // add antinodes if not in yet and if on map
                if !antinodes.contains(&antinode1)
                    && antinode1.0 >= 0
                    && antinode1.0 < field.len() as i32
                    && antinode1.1 >= 0
                    && antinode1.1 < field.get(0).unwrap().as_str().len() as i32
                {
                    antinodes.push(antinode1);
                }
                if !antinodes.contains(&antinode2)
                    && antinode2.0 >= 0
                    && antinode2.0 < field.len() as i32
                    && antinode2.1 >= 0
                    && antinode2.1 < field.get(0).unwrap().as_str().len() as i32
                {
                    antinodes.push(antinode2);
                }
            }
        }
    }

    println!("Result: {:?}", antinodes.len());
}
