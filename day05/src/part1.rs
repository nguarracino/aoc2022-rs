fn main() {
    let lines = Vec::from_iter(include_str!("../input.txt").lines());
    let position = lines.iter().position(|line| line.is_empty()).unwrap();
    let (stacks_input, moves_input) = lines.split_at(position);

    let mut stacks: Vec<Vec<String>> = stacks_input
        .iter()
        .map(|stack_input| Vec::from_iter(stack_input.split(' ').map(String::from)))
        .collect();

    let moves: Vec<Vec<String>> = moves_input
        .iter()
        .skip(1)
        .map(|move_input| Vec::from_iter(move_input.split(' ').map(String::from)))
        .collect();

    for m in moves {
        let count = m[1].parse::<usize>().unwrap();
        let from = m[3].parse::<usize>().unwrap() - 1;
        let to = m[5].parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            let popped = stacks[from].pop().unwrap();
            stacks[to].push(popped);
        }
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
}
