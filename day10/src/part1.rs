fn update_signal_boost(cycles: i32, register: i32) -> i32 {
    let signal_boost = if (cycles - 20) % 40 == 0 {
        cycles * register
    } else {
        0
    };
    signal_boost
}

fn main() {
    let lines = include_str!("../input.txt").lines();

    let mut cycles = 1;
    let mut register = 1;
    let mut signal_boost = 0;

    for line in lines {
        if let Ok(_) = sscanf::sscanf!(line, "noop") {
            signal_boost += update_signal_boost(cycles, register);
            cycles += 1;
        } else if let Ok(operand) = sscanf::sscanf!(line, "addx {}", i32) {
            for _ in 0..2 {
                signal_boost += update_signal_boost(cycles, register);
                cycles += 1;
            }
            register += operand;
        }
    }

    println!("signal_boost: {}", signal_boost);
}
