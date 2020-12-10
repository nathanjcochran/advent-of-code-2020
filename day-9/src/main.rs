use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("./data.txt").expect("Error reading file");
    let numbers = file
        .lines()
        .map(|i| i.parse::<i64>().expect("Invalid integer"))
        .collect();

    let answer = problem1(&numbers);
    problem2(&numbers, answer);
}

fn problem1(numbers: &Vec<i64>) -> i64 {
    println!("Problem 1");

    let mut map = HashMap::<i64, Vec<(usize, usize)>>::new();
    for (i, iv) in numbers.iter().enumerate() {
        for (j, jv) in numbers.iter().enumerate() {
            map.entry(iv + jv).or_insert(Vec::new()).push((i, j));
        }
    }

    for (idx, num) in numbers[25..].iter().enumerate() {
        let empty = Vec::new();
        let mut found = false;
        for (i, j) in map.get(num).unwrap_or(&empty) {
            let max = idx + 25;
            if i >= &idx && i < &max && j >= &idx && j < &max {
                found = true;
                break;
            }
        }
        if !found {
            println!("Answer: {}", num);
            return *num;
        }
    }
    panic!("Answer not found");
}

fn problem2(numbers: &Vec<i64>, target: i64) {
    println!("Problem 2");

    let mut lower = 0;
    let mut upper = 1;
    let mut sum = numbers[lower] + numbers[upper];
    loop {
        if sum < target {
            upper += 1;
            sum += numbers[upper];
        } else if sum > target {
            sum -= numbers[lower];
            lower += 1;
        } else {
            break;
        }
    }

    let mut min = numbers[lower];
    let mut max = numbers[lower];
    for num in numbers[lower..upper + 1].iter() {
        if *num < min {
            min = *num
        }
        if *num > max {
            max = *num
        }
    }

    println!("Answer: {}", min + max);
}
