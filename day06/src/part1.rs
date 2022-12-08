use std::collections::HashSet;

fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .into_iter()
        .next()
        .unwrap();

    let message_len = 4;
    let mut start = 0;
    let mut end = message_len;

    loop {
        let slice = lines.get(start..end);
        match slice {
            Some(s) => {
                let set: HashSet<char> = HashSet::from_iter(s.chars());
                if set.len() == message_len {
                    println!("{}", end);
                    break;
                }
            }
            _ => break,
        }
        start += 1;
        end += 1;
    }
}
