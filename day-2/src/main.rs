use std::fs;

struct Password {
    min: usize,
    max: usize,
    letter: String,
    password: String,
}

fn main() {
    let passwords: Vec<Password> = fs::read_to_string("./data.txt")
        .expect("Error reading file")
        .lines()
        .map(|s| {
            let parts: Vec<&str> = s.split(" ").collect();
            let rule_parts: Vec<usize> = parts[0]
                .split("-")
                .map(|s| s.parse().expect("Invalid integer"))
                .collect();
            let letter = parts[1].strip_suffix(":").unwrap();
            let password = parts[2];

            return Password {
                min: rule_parts[0],
                max: rule_parts[1],
                letter: letter.to_string(),
                password: password.to_string(),
            };
        })
        .collect();

    problem1(&passwords);
    problem2(&passwords);
}

fn problem1(passwords: &Vec<Password>) {
    println!("Problem 1");
    let valid = passwords
        .iter()
        .filter(|p| {
            let count = p.password.matches(&p.letter).count();
            return count >= p.min && count <= p.max;
        })
        .count();

    println!("Valid passwords: {}", valid)
}

fn problem2(passwords: &Vec<Password>) {
    println!("Problem 2");
    let valid = passwords
        .iter()
        .filter(|p| {
            let mut matches = 0;
            if p.password.as_bytes()[p.min - 1] == p.letter.as_bytes()[0] {
                matches += 1;
            }
            if p.password.as_bytes()[p.max - 1] == p.letter.as_bytes()[0] {
                matches += 1;
            }
            return matches == 1;
        })
        .count();

    println!("Valid passwords: {}", valid)
}
