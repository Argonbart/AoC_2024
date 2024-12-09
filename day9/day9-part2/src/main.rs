fn main() {
    // Read Input
    let input = include_str!("../input.txt");

    // Define Variables
    let mut number_blocks: Vec<i32> = Vec::new();
    let mut empty_blocks: Vec<i32> = Vec::new();
    let mut numbers_at: Vec<i64> = Vec::new();
    let mut number_id = 0;
    let mut offset = 0;

    // Read Input
    let mut is_even = true;
    const RADIX: u32 = 10;
    for i in 0..input.len() {
        // numbers
        if is_even {
            let current_number = input.chars().nth(i).unwrap().to_digit(RADIX).unwrap() as i32;
            number_blocks.push(current_number);
            empty_blocks.push(0);
            numbers_at.push(number_id);
            number_id += 1;
        }
        // free space
        else {
            let current_number = input.chars().nth(i).unwrap().to_digit(RADIX).unwrap() as i32;
            number_blocks.push(0);
            empty_blocks.push(current_number);
            numbers_at.push(-1);
        }
        is_even = !is_even;
    }

    // For every block starting from the back
    'outer: for next_block_index_counter in 0..number_blocks.len() {
        let next_block_index = number_blocks.len() - 1 - next_block_index_counter + offset;
        let block_size = number_blocks[next_block_index];
        if block_size == 0 {
            continue;
        }
        // Look for fitting gaps till the current index
        for next_gap_index_counter in 0..(empty_blocks.len() - next_block_index_counter) {
            let next_gap_index = next_gap_index_counter + offset;
            let gap_size = empty_blocks[next_gap_index];
            if gap_size == 0 {
                continue;
            }
            // Exact gap size = swap over
            if gap_size == block_size {
                numbers_at[next_gap_index] = numbers_at[next_block_index];
                number_blocks[next_gap_index] = number_blocks[next_block_index];
                empty_blocks[next_gap_index] = 0;

                numbers_at[next_block_index] = -1;
                number_blocks[next_block_index] = 0;
                empty_blocks[next_block_index] = block_size;
                continue 'outer;
            }
            // Bigger gap = insert and move offset
            if gap_size > block_size {
                numbers_at[next_gap_index] = numbers_at[next_gap_index];
                number_blocks[next_gap_index] = number_blocks[next_gap_index];
                empty_blocks[next_gap_index] = empty_blocks[next_gap_index] - block_size;

                numbers_at.insert(next_gap_index, numbers_at[next_block_index]);
                number_blocks.insert(next_gap_index, number_blocks[next_block_index]);
                empty_blocks.insert(next_gap_index, 0);
                offset += 1;

                numbers_at[next_block_index + 1] = -1;
                number_blocks[next_block_index + 1] = 0;
                empty_blocks[next_block_index + 1] = block_size;
                continue 'outer;
            }
        }
    }

    // Create output array
    let mut rearranged_vec: Vec<i64> = Vec::new();
    for i in 0..number_blocks.len() {
        if number_blocks[i] == 0 {
            for _j in 0..empty_blocks[i] {
                rearranged_vec.push(0);
            }
        } else {
            for _j in 0..number_blocks[i] {
                rearranged_vec.push(numbers_at[i]);
            }
        }
    }

    // Calculate sum of output array
    let mut sum: i64 = 0;
    let mut multiplier: i64 = 0;
    for digit in rearranged_vec.clone() {
        sum += digit * multiplier;
        multiplier += 1;
    }

    // Output Result
    println!("Result: {:?}", sum);
}
