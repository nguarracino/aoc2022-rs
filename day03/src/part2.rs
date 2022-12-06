fn item_priority(c: char) -> u16 {
    match c {
        'a'..='z' => c as u16 - 'a' as u16 + 1,
        'A'..='Z' => c as u16 - 'A' as u16 + 27,
        _ => 0,
    }
}
fn main() {
    let lines = Vec::from_iter(include_str!("../input.txt").lines());

    let mut total = 0;

    for group in lines.chunks(3) {
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                println!("{:?} {:?}", c, item_priority(c));
                total += item_priority(c);
                break;
            }
        }
    }

    println!("{:?}", total)
}
