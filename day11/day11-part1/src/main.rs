use count_digits::CountDigits;

fn main() {
    // Read Input
    let input = include_str!("../input.txt");
    let numbers: Vec<u128> = input
        .split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect();

    // Brute force through all numbers
    let mut input_stones: Vec<u128> = numbers.clone();
    let mut output_stones: Vec<u128> = Vec::new();
    let blink_amounts = 25;
    const RADIX: u32 = 10;
    let mut result = 0;
    for _blink in 0..blink_amounts {
        for stone in input_stones.iter() {
            // Case stone is zero
            if stone == &0 {
                output_stones.push(1);
            }
            // Case stone digits even amount
            else if stone.count_digits() % 2 == 0 {
                let stone_string: String = stone.to_string();
                let stone_chars = stone_string.chars();
                let stone_length = stone_chars.clone().count();
                let mut first_part: Vec<u128> = stone_chars
                    .clone()
                    .take(stone_length / 2)
                    .map(|x| x.to_digit(RADIX).unwrap() as u128)
                    .collect();

                let mut second_part: Vec<u128> = stone_chars
                    .clone()
                    .skip(stone_length / 2)
                    .map(|x| x.to_digit(RADIX).unwrap() as u128)
                    .collect();
                for elem in first_part.clone().into_iter() {
                    if elem == 0 {
                        first_part.remove(0);
                    } else {
                        break;
                    }
                }
                for elem in second_part.clone().into_iter() {
                    if elem == 0 {
                        second_part.remove(0);
                    } else {
                        break;
                    }
                }
                output_stones.push(first_part.iter().fold(0, |acc, elem| acc * 10 + elem));
                output_stones.push(second_part.iter().fold(0, |acc, elem| acc * 10 + elem));
            }
            // Case uneven amount
            else {
                output_stones.push(stone * 2024);
            }
        }
        // Save result and reset
        input_stones = output_stones.clone();
        result = output_stones.clone().iter().count();
        output_stones.clear();
    }

    // Output Result
    println!("Result: {:?}", result);
}
