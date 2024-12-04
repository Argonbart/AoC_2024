use regex::Regex;

fn main() {
    // Read input file
    let input = include_str!("../input.txt");

    // Create regex pattern to check for desired text bits
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    // Count appearances of that pattern
    // If do/dont then toggle boolean
    let mut counter = 0;
    let mut is_enabled = true;
    for match_ in pattern.captures_iter(input) {
        if match_.get(0).unwrap().as_str() == "do()" {
            is_enabled = true;
        } else if match_.get(0).unwrap().as_str() == "don't()" {
            is_enabled = false;
        } else if is_enabled {
            let num1 = match_.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let num2 = match_.get(2).unwrap().as_str().parse::<i32>().unwrap();
            counter += num1 * num2;
        }
    }

    // Output result
    println!("Result: {:?}", counter)
}
