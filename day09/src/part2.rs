use array2d::Array2D;
use std::convert::From;

#[derive(Clone, Debug)]
struct Coordinate {
    row: u32,
    col: u32,
}

impl Coordinate {
    fn is_touching(&self, other: &Coordinate) -> bool {
        self.col.abs_diff(other.col) < 2 && self.row.abs_diff(other.row) < 2
    }
    fn distance_to(&self, other: &Coordinate) -> (i32, i32) {
        (
            (other.row as i64 - self.row as i64).try_into().unwrap(),
            (other.col as i64 - self.col as i64).try_into().unwrap(),
        )
    }
    fn do_move(&mut self, m: &Move) {
        match m.direction.as_str() {
            "U" => {
                self.row += 1;
            }
            "D" => {
                self.row -= 1;
            }
            "L" => {
                self.col -= 1;
            }
            "R" => {
                self.col += 1;
            }
            _ => {
                panic!("Unknown direction {}", m.direction);
            }
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: String,
    steps: u32,
}

impl From<(String, u32)> for Move {
    fn from(m: (String, u32)) -> Self {
        Move {
            direction: m.0,
            steps: m.1,
        }
    }
}

fn drag_tail(rope: &mut Vec<Coordinate>) {
    for knot_pos_1 in 0..rope.len() - 1 {
        let knot_pos_2 = knot_pos_1 + 1;

        if rope[knot_pos_1].is_touching(&rope[knot_pos_2]) {
            continue;
        }

        let (row_diff, col_diff) = rope[knot_pos_1].distance_to(&rope[knot_pos_2]);
        if row_diff < 0 {
            rope[knot_pos_2].row += 1;
        } else if row_diff > 0 {
            rope[knot_pos_2].row -= 1;
        }

        if col_diff < 0 {
            rope[knot_pos_2].col += 1;
        } else if col_diff > 0 {
            rope[knot_pos_2].col -= 1;
        }
    }
}

fn main() {
    let lines: Vec<Move> = include_str!("../input.txt")
        .lines()
        .map(|line| Move::from(sscanf::sscanf!(line, "{} {}", String, u32).unwrap()))
        .collect();

    let mut grid = Array2D::filled_with(false, 1000, 1000);
    let mut rope = vec![Coordinate { row: 500, col: 500 }; 10];

    for m in lines {
        for _ in 0..m.steps {
            rope[0].do_move(&m);
            drag_tail(&mut rope);

            grid.set(
                rope.last().unwrap().row as usize,
                rope.last().unwrap().col as usize,
                true,
            )
            .unwrap();
        }
    }

    println!(
        "visited: {:?}",
        grid.as_column_major()
            .iter()
            .filter(|visited| **visited)
            .count()
    );
}
