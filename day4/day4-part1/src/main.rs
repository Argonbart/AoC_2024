fn main() {
    // Read input file
    let input = include_str!("../input.txt");
    let mut lines: Vec<&str> = Vec::new();
    for line in input.lines() {
        lines.push(line);
    }

    // Go through every line
    let mut xmas_counter = 0;
    for line_pos in 0..lines.len() {
        // Go through every character in the line
        for character_pos in 0..lines[0].len() {
            // Check for X
            if lines[line_pos].chars().nth(character_pos).unwrap() == 'X' {
                // Count XMAS for that X
                xmas_counter =
                    check_xmas(&lines, line_pos as i32, character_pos as i32, xmas_counter);
            }
        }
    }

    // Output result
    println!("Result: {:?}", xmas_counter)
}

fn check_xmas(lines: &Vec<&str>, line_pos: i32, character_pos: i32, xmas_counter: i32) -> i32 {
    // Set local counter
    let mut return_xmas_counter = xmas_counter;

    // Collect all directions from X that is valid and has a 'M'
    let mut valid_dirs: Vec<(i32, i32)> = Vec::new();
    for dir in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let check_line = line_pos + dir.0;
        let check_character = character_pos + dir.1;
        if in_bounds(&lines, check_line, check_character)
            && lines[(line_pos + dir.0 * 1) as usize]
                .chars()
                .nth((character_pos + dir.1 * 1) as usize)
                .unwrap()
                == 'M'
        {
            valid_dirs.push(dir);
        }
    }

    // Check in every valid direction for following 'A' and 'S'
    for dir in valid_dirs {
        if check_a_and_s(lines, line_pos, character_pos, dir) {
            return_xmas_counter += 1;
        }
    }

    // Return updated counter
    return return_xmas_counter;
}

// Check if position is inside the grid
fn in_bounds(lines: &Vec<&str>, check_line: i32, check_character: i32) -> bool {
    if check_line >= 0
        && check_line < lines.len() as i32
        && check_character >= 0
        && check_character < lines[0].len() as i32
    {
        true
    } else {
        false
    }
}

// Check if there is a 'A' and 'S' in the direction
fn check_a_and_s(
    lines: &Vec<&str>,
    line_pos_a: i32,
    character_pos_a: i32,
    dir: (i32, i32),
) -> bool {
    // Position of the 'A' in the direction
    let check_line_a = line_pos_a + dir.0 * 2;
    let check_character_a = character_pos_a + dir.1 * 2;

    // Position of the 'S' in the direction
    let check_line_s = line_pos_a + dir.0 * 3;
    let check_character_s = character_pos_a + dir.1 * 3;

    // Return if 'AS' follows
    if in_bounds(&lines, check_line_s, check_character_s)
        && lines[check_line_a as usize]
            .chars()
            .nth(check_character_a as usize)
            .unwrap()
            == 'A'
        && lines[check_line_s as usize]
            .chars()
            .nth(check_character_s as usize)
            .unwrap()
            == 'S'
    {
        true
    } else {
        false
    }
}
