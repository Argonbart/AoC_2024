use std::collections::HashMap;

fn main() {
    // Read Input
    let input = include_str!("../input.txt");
    let input_seperator = input.lines().position(|x| x.is_empty()).unwrap();

    // HashMap that save key = number to follow, value = previous numbers that had to appear before key
    let mut rules_lookup: HashMap<i32, Vec<i32>> = HashMap::new();

    // Read Rules
    for line in input.lines().take(input_seperator) {
        // Rules input finished
        if line.is_empty() {
            break;
        }

        // Read rule
        let mut rule = line.split("|");
        let previous_number = rule.next().unwrap().parse::<i32>().unwrap();
        let follow_number = rule.next().unwrap().parse::<i32>().unwrap();

        // Extend rule-lookup for the following number
        let mut right_list: Vec<i32> = Vec::new();
        if rules_lookup.contains_key(&previous_number) {
            right_list = rules_lookup.get(&previous_number).unwrap().to_vec();
        }
        right_list.push(follow_number);
        rules_lookup.insert(previous_number, right_list);
    }

    // Read updates
    let mut sum = 0;
    let mut valid_update = true;
    let mut update: Vec<i32>;
    let mut read_numbers: Vec<i32> = Vec::new();
    for line in input.lines().skip(input_seperator + 1) {
        // Update = Line with numbers
        update = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

        // Check rules for every line
        'outer: for number_position in 0..update.len() {
            let current_number = update.get(number_position).unwrap();
            for follow_number in rules_lookup.get(&current_number).unwrap().to_vec() {
                // If following number appeared before = Violation, so break
                if read_numbers.contains(&follow_number) {
                    valid_update = false;
                    break 'outer;
                }
            }
            // Add to list of numbers that appeared before
            read_numbers.push(*current_number);
        }

        // If no violation occured, add middle number
        if valid_update {
            sum += update.get(update.len() / 2).unwrap();
        }

        // Reset
        valid_update = true;
        update.clear();
        read_numbers.clear();
    }

    // Output result
    println!("Result: {:?}", sum);
}
