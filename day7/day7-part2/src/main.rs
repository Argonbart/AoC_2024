fn main() {
    // Read Input
    let input = include_str!("../input.txt");

    let mut sum = 0;
    for line in input.lines() {
        let mut split = line.split(": ");
        let target = split.next().unwrap().parse::<i64>().unwrap();
        let numbers_string = split.next().unwrap();
        let numbers: Vec<i64> = numbers_string
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        if test_plus(0, &numbers, target)
            || test_mult(0, &numbers, target)
            || test_concat(0, &numbers, target)
        {
            sum += target;
        }
    }
    dbg!(sum);
}

fn test_plus(left_sum: i64, numbers: &Vec<i64>, target: i64) -> bool {
    let mut left_sum = left_sum.clone();
    let mut numbers = numbers.clone();
    if numbers.len() == 0 {
        return left_sum == target;
    }
    left_sum += numbers.remove(0);
    return test_plus(left_sum, &numbers, target)
        || test_mult(left_sum, &numbers, target)
        || test_concat(left_sum, &numbers, target);
}

fn test_mult(left_sum: i64, numbers: &Vec<i64>, target: i64) -> bool {
    let mut left_sum = left_sum.clone();
    let mut numbers = numbers.clone();
    if numbers.len() == 0 {
        return left_sum == target;
    }
    left_sum *= numbers.remove(0);
    return test_plus(left_sum, &numbers, target)
        || test_mult(left_sum, &numbers, target)
        || test_concat(left_sum, &numbers, target);
}

fn test_concat(left_sum: i64, numbers: &Vec<i64>, target: i64) -> bool {
    let mut left_sum = left_sum.clone();
    let mut numbers = numbers.clone();
    if numbers.len() == 0 {
        return left_sum == target;
    }
    left_sum = (left_sum.to_string() + numbers.remove(0).to_string().as_str())
        .parse::<i64>()
        .unwrap();
    return test_plus(left_sum, &numbers, target)
        || test_mult(left_sum, &numbers, target)
        || test_concat(left_sum, &numbers, target);
}
