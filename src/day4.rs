use aoc_runner_derive::aoc;
use std::collections::HashSet;

type Row = i32;
type Column = i32;

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let grid = parse_input(input);

    let mut result = 0;
    for coordinate in &grid {
        let mut neighbours = 0;
        for delta_row in -1..=1 {
            for delta_column in -1..=1 {
                if delta_row == 0 && delta_column == 0 {
                    continue;
                }
                if grid.contains(&Coordinate(
                    coordinate.0 + delta_row,
                    coordinate.1 + delta_column,
                )) {
                    neighbours += 1;
                }
            }
        }
        if neighbours < 4 {
            result += 1;
        }
    }

    result
}

#[allow(dead_code)]
fn print_grid(grid: &HashSet<Coordinate>) {
    let (row_min, col_min, row_max, col_max) = get_boundaries(grid);
    for row in row_min..=row_max {
        for column in col_min..=col_max {
            if grid.contains(&Coordinate(row, column)) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn get_boundaries(grid: &HashSet<Coordinate>) -> (Row, Column, Row, Column) {
    grid.iter().fold(
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        |(row_min, col_min, row_max, col_max), coordinate| {
            let Coordinate(other_row, other_column) = coordinate;

            (
                row_min.min(*other_row),
                col_min.min(*other_column),
                row_max.max(*other_row),
                col_max.max(*other_column),
            )
        },
    )
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinate(Row, Column);

fn parse_input(input: &str) -> HashSet<Coordinate> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(column, char)| match char {
                    '@' => Some(Coordinate(
                        i32::try_from(row).unwrap(),
                        i32::try_from(column).unwrap(),
                    )),
                    _ => None,
                })
        })
        .collect::<HashSet<Coordinate>>()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let mut grid = parse_input(input);

    let initial_number_of_rolls = grid.len();
    loop {
        let mut coords_to_remove = vec![];
        for coordinate in &grid {
            let mut neighbours = 0;
            for delta_row in -1..=1 {
                for delta_column in -1..=1 {
                    if delta_row == 0 && delta_column == 0 {
                        continue;
                    }
                    if grid.contains(&Coordinate(
                        coordinate.0 + delta_row,
                        coordinate.1 + delta_column,
                    )) {
                        neighbours += 1;
                    }
                }
            }
            if neighbours < 4 {
                coords_to_remove.push(*coordinate);
            }
        }
        if coords_to_remove.is_empty() {
            break;
        }
        for coord_to_remove in coords_to_remove {
            let _ = grid.remove(&coord_to_remove);
        }
    }

    initial_number_of_rolls - grid.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2025/day4.txt");
        assert_eq!(part1(input), 1367);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2025/day4.txt");
        assert_eq!(part2(input), 9144);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#,
        );

        assert_eq!(result, 13)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#,
        );

        assert_eq!(result, 43)
    }
}
