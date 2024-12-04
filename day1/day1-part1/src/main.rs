fn main() {
    // Read input file
    let input = include_str!("../input.txt");

    // Create arrays for left and right column
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    // Fill arrays line by line
    for line in input.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        left_numbers.push(numbers[0].parse::<i32>().unwrap());
        right_numbers.push(numbers[1].parse::<i32>().unwrap());
    }

    // Sort arrays
    left_numbers.sort();
    right_numbers.sort();

    // Add up differences
    let mut sum = 0;
    for i in 0..left_numbers.len() {
        sum += (right_numbers[i] - left_numbers[i]).abs()
    }

    // Output result
    println!("Result: {:?}", sum);
}
