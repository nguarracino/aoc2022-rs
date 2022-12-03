fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<u32>());

    let mut elf = 0;
    let mut max: [u32; 3] = [0, 0, 0];

    for line in lines.into_iter() {
        match line {
            Ok(n) => elf += n,
            _ => {
                if elf > max[0] {
                    max[0] = elf;
                    max.sort();
                }
                elf = 0;
            }
        }
    }

    println!("{:?}", max.iter().sum::<u32>());
}
