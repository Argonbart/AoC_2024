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

    // Count number and add multiplication
    let mut sum = 0;
    for number in left_numbers {
        let number_count = right_numbers.iter().filter(|&x| *x == number).count() as i32;
        sum += number * number_count;
    }

    // Output result
    println!("Result: {:?}", sum);
}
