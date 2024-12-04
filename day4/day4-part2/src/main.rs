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
            // Check for A
            if check_for_letter(&lines, line_pos, character_pos, 'A') {
                // Check if there is a X-MAS for that A
                if check_x_mas(&lines, line_pos as i32, character_pos as i32) {
                    xmas_counter += 1;
                }
            }
        }
    }

    // Output result
    println!("Result: {:?}", xmas_counter)
}

fn check_x_mas(lines: &Vec<&str>, line_pos: i32, character_pos: i32) -> bool {
    // Define diagonals
    let diagonal1: ((i32, i32), (i32, i32)) = (
        (line_pos - 1, character_pos - 1),
        (line_pos + 1, character_pos + 1),
    );
    let diagonal2: ((i32, i32), (i32, i32)) = (
        (line_pos - 1, character_pos + 1),
        (line_pos + 1, character_pos - 1),
    );

    //Check if diagonals are in bounds
    let mut diagonal1_is_in_bound = true;
    let mut diagonal2_is_in_bound = true;
    if !diagonal_in_bounds(lines, diagonal1) {
        diagonal1_is_in_bound = false;
    }
    if !diagonal_in_bounds(lines, diagonal2) {
        diagonal2_is_in_bound = false;
    }

    // Check for MS or SM on both diagonals
    let mut diagonal1_is_mas = false;
    let mut diagonal2_is_mas = false;
    if diagonal1_is_in_bound {
        if (check_for_letter(
            &lines,
            diagonal1.0 .0 as usize,
            diagonal1.0 .1 as usize,
            'M',
        ) && check_for_letter(
            &lines,
            diagonal1.1 .0 as usize,
            diagonal1.1 .1 as usize,
            'S',
        )) || (check_for_letter(
            &lines,
            diagonal1.0 .0 as usize,
            diagonal1.0 .1 as usize,
            'S',
        ) && check_for_letter(
            &lines,
            diagonal1.1 .0 as usize,
            diagonal1.1 .1 as usize,
            'M',
        )) {
            diagonal1_is_mas = true;
        }
    }
    if diagonal2_is_in_bound {
        if (check_for_letter(
            &lines,
            diagonal2.0 .0 as usize,
            diagonal2.0 .1 as usize,
            'M',
        ) && check_for_letter(
            &lines,
            diagonal2.1 .0 as usize,
            diagonal2.1 .1 as usize,
            'S',
        )) || (check_for_letter(
            &lines,
            diagonal2.0 .0 as usize,
            diagonal2.0 .1 as usize,
            'S',
        ) && check_for_letter(
            &lines,
            diagonal2.1 .0 as usize,
            diagonal2.1 .1 as usize,
            'M',
        )) {
            diagonal2_is_mas = true;
        }
    }

    // Return true if double MAS found
    return diagonal1_is_mas && diagonal2_is_mas;
}

// Check if both diagonal points are in bound
fn diagonal_in_bounds(lines: &Vec<&str>, diagonal: ((i32, i32), (i32, i32))) -> bool {
    return in_bounds(lines, diagonal.0 .0, diagonal.0 .1)
        && in_bounds(lines, diagonal.1 .0, diagonal.1 .1);
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

// Check if letter is at position
fn check_for_letter(
    lines: &Vec<&str>,
    line_pos: usize,
    character_pos: usize,
    letter: char,
) -> bool {
    return lines[line_pos].chars().nth(character_pos).unwrap() == letter;
}
