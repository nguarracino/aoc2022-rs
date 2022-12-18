fn print_pixel(register: i32, cycles: i32) {
    if (register - 1..=register + 1).contains(&(cycles - 1)) {
        print!("#");
    } else {
        print!(".");
    }
}

fn adjust_cycles(cycles: i32) -> i32 {
    let mut new_cycles = cycles + 1;
    if new_cycles == 41 {
        println!();
        new_cycles = 1;
    }
    new_cycles
}

fn main() {
    let lines = include_str!("../input.txt").lines();

    let mut cycles = 1;
    let mut register = 1;

    for line in lines {
        if let Ok(_) = sscanf::sscanf!(line, "noop") {
            print_pixel(register, cycles);
            cycles = adjust_cycles(cycles);
        } else if let Ok(operand) = sscanf::sscanf!(line, "addx {}", i32) {
            for _ in 0..2 {
                print_pixel(register, cycles);
                cycles = adjust_cycles(cycles);
            }
            register += operand;
        }
    }
}
