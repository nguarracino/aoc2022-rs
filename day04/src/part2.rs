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
            let vec1: Vec<u32> = (start1..=end1).collect();
            let set1: HashSet<u32> = HashSet::from_iter(vec1);

            let start2 = line[1][0];
            let end2 = line[1][1];
            let vec2: Vec<u32> = (start2..=end2).collect();
            let set2: HashSet<u32> = HashSet::from_iter(vec2);

            set1.intersection(&set2).count() > 0
        })
        .count();

    println!("{:?}", count);
}
