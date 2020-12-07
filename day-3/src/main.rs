use std::fs;

fn main() {
    let file = fs::read_to_string("./data.txt").expect("Error reading file");
    let lines: Vec<&[u8]> = file.lines().map(|s| s.as_bytes()).collect();

    problem1(&lines);
    problem2(&lines);
}

fn problem1(lines: &Vec<&[u8]>) {
    println!("Problem 1");
    let trees = lines
        .iter()
        .enumerate()
        .filter(|(i, l)| l[(i * 3) % l.len()] as char == '#')
        .count();

    println!("Trees: {}", trees)
}

fn problem2(lines: &Vec<&[u8]>) {
    println!("Problem 2");
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut trees = 1;
    for slope in &slopes {
        trees *= lines
            .iter()
            .enumerate()
            .filter(|(i, l)| {
                l[((i * slope.0) / slope.1) % l.len()] as char == '#' && i % slope.1 == 0
            })
            .count();
    }

    println!("Trees: {}", trees)
}
