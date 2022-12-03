fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<u32>());

    let mut elf= 0;
    let mut max = 0;

    for line in lines.into_iter() {
        match line {
            Ok(n) => elf += n,
            _ => {
                if elf > max {
                    max = elf;
                }
                elf = 0;
            }
        }
    };
    
    println!("{:?}", max);
}
