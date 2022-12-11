use std::collections::HashMap;

fn main() {
    let lines = include_str!("../input.txt").lines();

    let mut curr_path = Vec::new();
    let mut dir_sizes = HashMap::new();

    for line in lines {
        if let Ok(dir) = sscanf::sscanf!(line, "$ cd {}", str) {
            match dir {
                ".." => {
                    curr_path.pop();
                }
                _ => {
                    curr_path.push(dir);
                }
            }
        } else if let Ok((file_size, _)) = sscanf::sscanf!(line, "{} {}", usize, str) {
            for i in 0..curr_path.len() {
                let path = curr_path[0..i+1].join("/");
                let curr_size = dir_sizes.get(&path).unwrap_or(&0);
                dir_sizes.insert(path, curr_size + file_size);
            }
        }
    }

    let free_space = 70000000 - dir_sizes.get("/").unwrap_or(&0);
    let needed_space = 30000000 - free_space;

    let small_dirs: Vec<usize> = dir_sizes.iter().filter(|dir_size| {
        dir_size.1 >= &needed_space
    }).map(|dir_size| {
        *dir_size.1
    }).collect();

    println!("{:?}", small_dirs.iter().min().unwrap_or(&0));
}
