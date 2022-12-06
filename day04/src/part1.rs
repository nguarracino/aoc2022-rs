use std::collections::HashSet;

fn main() {
    let lines: Vec<Vec<Vec<u32>>> = include_str!("../input.txt")
        .lines()
        .map(|l1| {
            Vec::from_iter(
                l1.split(',')
                    .map(|l2| Vec::from_iter(l2.split('-').map(|l3| l3.parse::<u32>().unwrap()))),
            )
        })
        .collect();

    let count = lines
        .iter()
        .filter(|line| {
            let start1 = line[0][0];
            let end1 = line[0][1];
            let set1: HashSet<u32> = HashSet::from_iter(start1..=end1);

            let start2 = line[1][0];
            let end2 = line[1][1];
            let set2: HashSet<u32> = HashSet::from_iter(start2..=end2);

            set1.is_subset(&set2) || set2.is_subset(&set1)
        })
        .count();

    println!("{:?}", count);
}
