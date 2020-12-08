use std::collections::HashMap;
use std::fs;

struct Bag {
    count: u32,
    color: String,
}

fn main() {
    let file = fs::read_to_string("./data.txt").expect("Error reading file");
    let rules = file
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" bags contain ").collect();
            if parts[1] == "no other bags." {
                return (parts[0], Vec::<Bag>::new());
            }
            let contents: Vec<Bag> = parts[1]
                .split(", ")
                .map(|b| {
                    let b = b
                        .trim_end_matches(".")
                        .trim_end_matches(" bags")
                        .trim_end_matches(" bag");
                    let parts: Vec<&str> = b.split_whitespace().collect();
                    let count = parts[0].parse::<u32>().expect("Invalid integer");
                    Bag {
                        count: count,
                        color: parts[1..].join(" "),
                    }
                })
                .collect();

            return (parts[0], contents);
        })
        .collect();

    problem1(&rules);
}

fn problem1(rules: &Vec<(&str, Vec<Bag>)>) {
    println!("Problem 1");

    let mut graph = HashMap::<&str, Vec<&str>>::new();
    for rule in rules {
        for bag in &rule.1 {
            match graph.get_mut(&bag.color.as_str()) {
                Some(vec) => {
                    vec.push(rule.0);
                }
                _ => {
                    graph.insert(&bag.color, vec![rule.0]);
                }
            };
        }
    }

    let mut found = HashMap::<&str, bool>::new();
    let mut cur = vec!["shiny gold"];
    let mut next = Vec::<&str>::new();
    let mut count = 0;
    while cur.len() > 0 {
        for c in cur {
            match graph.get(c) {
                Some(vec) => {
                    for v in vec {
                        if found.contains_key(v) {
                            continue;
                        }
                        found.insert(&v, true);
                        next.push(&v);
                        count += 1;
                    }
                }
                _ => {}
            }
        }
        cur = next;
        next = Vec::<&str>::new();
    }

    println!("Bag colors: {}", count);
}
