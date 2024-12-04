use regex::Regex;

fn main() {
    // Read input file
    let input = include_str!("../input.txt");

    // Create regex pattern to check for desired text bits
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Count appearances of that pattern
    let mut counter = 0;
    for match_ in pattern.captures_iter(input) {
        let num1 = match_.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let num2 = match_.get(2).unwrap().as_str().parse::<i32>().unwrap();
        counter += num1 * num2;
    }

    // Output result
    println!("Result: {:?}", counter)
}
