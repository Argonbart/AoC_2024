fn main() {
    // Read input file
    let input = include_str!("../input.txt");

    // Check and count safe reports line by line
    let mut safe_counter = 0;
    for line in input.lines() {
        // Split string into numbers
        let report_numbers = line.split(" ");

        // Test for increasing numbers
        let report_safety_increasing = report_numbers
            .clone()
            .fold(
                (
                    true,
                    report_numbers
                        .clone()
                        .next()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap()
                        - 1,
                ),
                |acc, num| {
                    let num = num.parse::<i32>().unwrap();
                    if acc.0 && num > acc.1 && num < acc.1 + 4 {
                        (true, num)
                    } else {
                        (false, num)
                    }
                },
            )
            .0;

        // Test for decreasing numbers
        let report_safety_decreasing = report_numbers
            .clone()
            .fold(
                (
                    true,
                    report_numbers
                        .clone()
                        .next()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap()
                        + 1,
                ),
                |acc, num| {
                    let num = num.parse::<i32>().unwrap();
                    if acc.0 && num < acc.1 && num > acc.1 - 4 {
                        (true, num)
                    } else {
                        (false, num)
                    }
                },
            )
            .0;

        // If any worked, add to counter
        if report_safety_increasing || report_safety_decreasing {
            safe_counter += 1;
        }
    }

    // Output result
    println!("Number of safe reports: {:?}", safe_counter);
}
