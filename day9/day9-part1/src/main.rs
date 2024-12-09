fn main() {
    // Read Input
    const INPUT: &str = include_str!("../input.txt");

    // Define Variables
    let mut input_vec: Vec<i32> = Vec::new();

    // Read Input
    let mut is_even = true;
    let mut index = 0;
    const RADIX: u32 = 10;
    for i in 0..INPUT.len() {
        // numbers
        if is_even {
            for _i in 0..INPUT.chars().nth(i).unwrap().to_digit(RADIX).unwrap() {
                input_vec.push(index);
            }
            index += 1;
        }
        // free space
        else {
            for _i in 0..INPUT.chars().nth(i).unwrap().to_digit(RADIX).unwrap() {
                input_vec.push(-1);
            }
        }
        is_even = !is_even;
    }

    // Go through input_vec
    // If number take it, if dot then swap with the last number
    // Stop when input_vec has only dots from current position left
    let mut rearranged_vec: Vec<u128> = Vec::new();
    for i in 0..input_vec.len() {
        let mut only_dots = true;
        let remaining_part = input_vec.iter().skip(i);
        for item in remaining_part {
            if *item != -1 {
                only_dots = false;
            }
        }
        if only_dots {
            break;
        }
        if *input_vec.get(i).unwrap() == -1 {
            // swap
            for j in (0..input_vec.len()).rev() {
                let current_number = *input_vec.get(j).unwrap();
                if current_number != -1 {
                    rearranged_vec.push(current_number as u128);
                    input_vec[j] = -1;
                    break;
                }
            }
        } else {
            rearranged_vec.push(*input_vec.get(i).unwrap() as u128);
        }
    }

    // Calculate sum
    let mut sum: u128 = 0;
    let mut multiplier: u128 = 0;
    for digit in rearranged_vec.clone() {
        sum += digit * multiplier;
        multiplier += 1;
    }

    // Output Result
    println!("Rearranged Line: {:?}", rearranged_vec);
    println!("Result: {:?}", sum);
}
