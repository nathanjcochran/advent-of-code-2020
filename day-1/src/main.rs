use std::fs;

fn main() {
    let numbers: Vec<i32> = fs::read_to_string("./data.txt")
        .expect("Error reading file")
        .lines()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();

    problem1(&numbers);
    problem2(&numbers);
}

fn problem1(numbers: &Vec<i32>) {
    println!("Problem 1");
    for i in numbers {
        for j in numbers {
            if i + j == 2020 {
                println!("i: {}", i);
                println!("j: {}", j);
                println!("answer: {}", i * j);
                return;
            }
        }
    }
}

fn problem2(numbers: &Vec<i32>) {
    println!("Problem 2");
    for i in numbers {
        for j in numbers {
            for k in numbers {
                if i + j + k == 2020 {
                    println!("i: {}", i);
                    println!("j: {}", j);
                    println!("k: {}", k);
                    println!("answer: {}", i * j * k);
                    return;
                }
            }
        }
    }
}
