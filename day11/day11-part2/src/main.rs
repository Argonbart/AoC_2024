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
    let blink_amounts = 75;
    let mut result: Vec<u128> = [1].to_vec();
    let mut step_counter = 0;
    println!("Step {:?}: {:?}", step_counter, result);
    step_counter += 1;

    // Caching

    for _blink in 0..blink_amounts {
        for stone in input_stones.iter() {
            if stone == &0 {
                output_stones.push(1);
            }
            if stone.count_digits() % 2 == 0 {
                const RADIX: u32 = 10;
                let stone_string: String = stone.to_string();
                let stone_chars = stone_string.chars();
                let stone_length = stone_chars.clone().count();
                let first_part: Vec<u128> = stone_chars
                    .clone()
                    .take(stone_length / 2)
                    .map(|x| x.to_digit(RADIX).unwrap() as u128)
                    .collect();

                let second_part: Vec<u128> = stone_chars
                    .clone()
                    .skip(stone_length / 2)
                    .map(|x| x.to_digit(RADIX).unwrap() as u128)
                    .collect();
                let first_number = first_part.iter().fold(0, |acc, elem| acc * 10 + elem);
                let second_number = second_part.iter().fold(0, |acc, elem| acc * 10 + elem);
                output_stones.push(first_number);
                output_stones.push(second_number);
            } else {
                output_stones.push(stone * 2024);
            }
        }
        result = output_stones.clone();
        println!("Step {:?}", step_counter);
        step_counter += 1;
        input_stones = output_stones.clone();
        output_stones.clear();
    }

    // Output Result
    // println!("Result: {:?}", result);
}

// 89741 ->
// 316108 ->
// 7641 -> 7 6 4 1
// 756 -> 6 2 6 8 3 2 8 0 2 3 1 8 6 9 4 4
// 7832357 ->
// 91 -> 9 1

// ---------------------

// STEP
// 0 - 1     - 2        - 3         - 4           - 5

// 0 - 1     -          -           -             -
// 1 - 2024  - 20 24    - 2 0 2 4   -             -
// 2 - 4048  - 40 48    - 4 0 4 8   -             -
// 3 - 6072  - 60 72    - 6 0 7 2   -             -
// 4 - 8096  - 80 96    - 8 0 9 6   -             -

// 5 - 10120 - 20482880 - 2048 2880 - 20 48 28 80 - 2 0 4 8 2 8 8 0
// 6 - 12144 - 24579456 - 2457 9456 - 24 57 94 56 - 2 4 5 7 9 4 5 6
// 7 - 14168 - 28676032 - 2867 6032 - 28 67 60 32 - 2 8 6 7 6 0 3 2
// 8 - 16192 - 32772608 - 3277 2608 - 32 77 26 08 - 3 2 7 7 2 6 0 8
// 9 - 18216 - 36869184 - 3686 9184 - 36 86 91 84 - 3 6 8 6 9 1 8 4
