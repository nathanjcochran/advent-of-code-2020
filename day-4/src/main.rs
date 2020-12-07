use std::fs;

struct Field {
    name: String,
    value: String,
}

fn main() {
    let file = fs::read_to_string("./data.txt").expect("Error reading file");
    let batches = file
        .split("\n\n")
        .map(|b| {
            b.split_whitespace()
                .map(|f| f.split(":").collect())
                .map(|f: Vec<&str>| Field {
                    name: f[0].to_string(),
                    value: f[1].to_string(),
                })
                .collect()
        })
        .collect();

    problem1(&batches);
    problem2(&batches);
}

fn problem1(batches: &Vec<Vec<Field>>) {
    println!("Problem 1");
    let valid = batches
        .iter()
        .filter(|b| {
            b.iter()
                .filter(|f| match f.name.as_str() {
                    "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => true,
                    _ => false,
                })
                .count()
                >= 7
        })
        .count();

    println!("Valid: {}", valid)
}

fn problem2(batches: &Vec<Vec<Field>>) {
    println!("Problem 2");
    let valid = batches
        .iter()
        .filter(|b| {
            b.iter()
                .filter(|f| match f.name.as_str() {
                    "byr" => match f.value.parse::<u32>() {
                        Ok(val) => val >= 1920 && val <= 2002,
                        Err(_) => false,
                    },
                    "iyr" => match f.value.parse::<u32>() {
                        Ok(val) => val >= 2010 && val <= 2020,
                        Err(_) => false,
                    },
                    "eyr" => match f.value.parse::<u32>() {
                        Ok(val) => val >= 2020 && val <= 2030,
                        Err(_) => false,
                    },
                    "hgt" => {
                        if f.value.ends_with("cm") {
                            match f.value.strip_suffix("cm").unwrap().parse::<u32>() {
                                Ok(val) => val >= 150 && val <= 193,
                                Err(_) => false,
                            }
                        } else if f.value.ends_with("in") {
                            match f.value.strip_suffix("in").unwrap().parse::<u32>() {
                                Ok(val) => val >= 59 && val <= 76,
                                Err(_) => false,
                            }
                        } else {
                            false
                        }
                    }
                    "hcl" => {
                        if !f.value.starts_with("#") {
                            false
                        } else {
                            f.value[1..]
                                .chars()
                                .filter(|c| (c >= &'0' && c <= &'9') || (c >= &'a' && c <= &'f'))
                                .count()
                                == 6
                        }
                    }
                    "ecl" => match f.value.as_str() {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                        _ => false,
                    },
                    "pid" => f.value.chars().filter(|c| c >= &'0' && c <= &'9').count() == 9,
                    _ => false,
                })
                .count()
                >= 7
        })
        .count();

    println!("Valid: {}", valid)
}
