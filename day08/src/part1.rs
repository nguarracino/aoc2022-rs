use array2d::Array2D;

fn is_visible(grid: &Array2D<u16>, row_num: usize, col_num: usize) -> bool {
    let tree_height = grid.get(row_num, col_num).unwrap();

    sight_lines(grid, row_num, col_num)
        .iter()
        .any(|trees| trees.iter().all(|&tree| tree < tree_height))
}

fn sight_lines(grid: &Array2D<u16>, row_num: usize, col_num: usize) -> Vec<Vec<&u16>> {
    let col = Vec::from_iter(grid.column_iter(col_num).unwrap());
    let mut up = Vec::from(&col[..row_num]);
    up.reverse();
    let down = Vec::from(&col[row_num + 1..]);

    let row = Vec::from_iter(grid.row_iter(row_num).unwrap());
    let mut left = Vec::from(&row[..col_num]);
    left.reverse();
    let right = Vec::from(&row[col_num + 1..]);

    [up, down, left, right].to_vec()
}

fn main() {
    let lines: Vec<Vec<u16>> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            Vec::from_iter(
                line.chars()
                    .map(|c| String::from(c).parse::<u16>().unwrap()),
            )
        })
        .collect();

    let grid = Array2D::from_rows(&lines).unwrap();
    let mut visible_grid = Array2D::filled_with(false, grid.num_rows(), grid.num_columns());

    for row in 0..grid.num_rows() {
        for col in 0..grid.num_columns() {
            visible_grid
                .set(row, col, is_visible(&grid, row, col))
                .unwrap();
        }
    }

    println!(
        "visible_trees: {:?}",
        visible_grid
            .as_column_major()
            .iter()
            .filter(|visible| **visible)
            .count()
    );
}
