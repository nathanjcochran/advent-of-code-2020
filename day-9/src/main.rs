use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("./data.txt").expect("Error reading file");
    let numbers = file
        .lines()
        .map(|i| i.parse::<i64>().expect("Invalid integer"))
        .collect();

    problem1(&numbers);
}

fn problem1(numbers: &Vec<i64>) {
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
            break;
        }
    }
}
