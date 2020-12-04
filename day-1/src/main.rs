use std::fs;

fn main() {
    let numbers: Vec<i32> = fs::read_to_string("./data.txt")
        .expect("Error reading file")
        .lines()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();

    for i in &numbers {
        for j in &numbers {
            if i + j == 2020 {
                println!("i: {}", i);
                println!("j: {}", j);
                println!("answer: {}", i * j);
                return;
            }
        }
    }
}
