fn item_priority(c: char) -> u16 {
    match c {
        'a'..='z' => c as u16 - 'a' as u16 + 1,
        'A'..='Z' => c as u16 - 'A' as u16 + 27,
        _ => 0
    }
}
fn main() {
    let lines: Vec<(&str, &str)> = include_str!("../input.txt")
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect();

    let mut total = 0;

    for line in lines.iter() {
        for c in line.0.chars() {
            if line.1.contains(c) {
                println!("{:?} {:?}", c, item_priority(c));
                total += item_priority(c);
                break;
            }
        }
    }

    println!("{:?}", total)
}
