use std::collections::HashMap;

use count_digits::CountDigits;

fn main() {
    // Read Input
    let input = include_str!("../input.txt");
    let numbers: Vec<i64> = input
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    // Variables
    let mut input_dict: HashMap<i64, i64> = HashMap::new();
    let mut output_dict: HashMap<i64, i64> = HashMap::new();
    let blink_amounts = 75;
    let mut result: i64 = 0;

    // Fill first input_dict
    for number in numbers {
        if input_dict.contains_key(&number) {
            *input_dict.get_mut(&number).unwrap() += 1;
        } else {
            input_dict.insert(number, 1);
        }
    }

    // Go blink by blink
    for _blink in 0..blink_amounts {
        // Check every dict entry, perform tranformation for whole bunch of numbers
        for entry in input_dict.iter() {
            let input_number = entry.0;
            let input_number_amount = entry.1;
            // dict entry is zero
            if input_number == &0 {
                let new_number = 1;
                if output_dict.contains_key(&new_number) {
                    *output_dict.get_mut(&new_number).unwrap() += input_number_amount;
                } else {
                    output_dict.insert(new_number, *input_number_amount);
                }
            }
            // dict entry has even amount of digits
            else if input_number.count_digits() % 2 == 0 {
                let stone_string = input_number.to_string();
                let stone_length = stone_string.len();
                let first_string = &stone_string[..stone_length / 2];
                let second_string = &stone_string[(stone_length / 2)..stone_length];
                let first_number = first_string.parse::<i64>().unwrap();
                let second_number = second_string.parse::<i64>().unwrap();

                let new_number = first_number;
                if output_dict.contains_key(&new_number) {
                    *output_dict.get_mut(&new_number).unwrap() += input_number_amount;
                } else {
                    output_dict.insert(new_number, *input_number_amount);
                }

                let new_number = second_number;
                if output_dict.contains_key(&new_number) {
                    *output_dict.get_mut(&new_number).unwrap() += input_number_amount;
                } else {
                    output_dict.insert(new_number, *input_number_amount);
                }
            }
            // dict entry has odd amount of digits
            else {
                let new_number = input_number * 2024;
                if output_dict.contains_key(&new_number) {
                    *output_dict.get_mut(&new_number).unwrap() += input_number_amount;
                } else {
                    output_dict.insert(new_number, *input_number_amount);
                }
            }
        }
        // Reset dict for next blink
        input_dict = output_dict.clone();
        output_dict.clear();
    }

    // Count final step dict counts
    for entry in input_dict.values() {
        result += entry;
    }

    // Output Result
    println!("Result: {:?}", result);
}
