use crate::day9::Direction::East;
use crate::day9::Direction::North;
use crate::day9::Direction::South;
use crate::day9::Direction::West;
use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
struct Corner {
    column: i64,
    row: i64,
}

#[aoc(day9, part1)]
fn part1(input: &str) -> i64 {
    let corners = input
        .lines()
        .map(|line| {
            let coords = line
                .split(',')
                .map(|d| d.parse().unwrap())
                .collect::<Vec<_>>();
            Corner {
                column: coords[0],
                row: coords[1],
            }
        })
        .collect::<Vec<_>>();

    corners
        .iter()
        .permutations(2)
        .map(|perms| {
            let left = perms[0];
            let right = perms[1];

            let rectangle = Rectangle {
                col_max: left.column.max(right.column),
                col_min: left.column.min(right.column),
                row_max: left.row.max(right.row),
                row_min: left.row.min(right.row),
            };

            rectangle.area()
        })
        .max()
        .unwrap()
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    col_max: i64,
    col_min: i64,
    row_max: i64,
    row_min: i64,
}

impl Rectangle {
    const fn area(&self) -> i64 {
        (self.col_max - self.col_min + 1) * (self.row_max - self.row_min + 1)
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    column: i64,
    row: i64,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Cell {
    Corner,
    Border,
    Padding,
}

#[aoc(day9, part2)]
fn part2(input: &str) -> i64 {
    solve_part2(input, 100_000)
}

#[allow(clippy::too_many_lines)]
fn solve_part2(input: &str, skips: usize) -> i64 {
    let mut corners = input
        .lines()
        .map(|line| {
            let coords = line
                .split(',')
                .map(|d| d.parse().unwrap())
                .collect::<Vec<_>>();
            Corner {
                column: coords[0],
                row: coords[1],
            }
        })
        .collect::<Vec<_>>();
    let rectangles = corners
        .iter()
        .permutations(2)
        .map(|perms| {
            let left = perms[0];
            let right = perms[1];

            Rectangle {
                col_max: left.column.max(right.column),
                col_min: left.column.min(right.column),
                row_max: left.row.max(right.row),
                row_min: left.row.min(right.row),
            }
        })
        .sorted_by_key(Rectangle::area)
        .rev()
        .collect::<Vec<_>>();
    let mut border = vec![];

    corners.push(*corners.first().unwrap());

    for window in corners.windows(2) {
        let c1 = window[0].column;
        let c2 = window[1].column;
        let r1 = window[0].row;
        let r2 = window[1].row;

        if c1 == c2 {
            for row in r1.min(r2)..=(r1.max(r2)) {
                // println!("\tPoint at col {c1} row {row}");
                border.push(Point { column: c1, row });
            }
        } else if r1 == r2 {
            for column in c1.min(c2)..=(c1.max(c2)) {
                border.push(Point { column, row: r1 });
            }
        } else {
            panic!()
        }
    }

    let mut grid: HashMap<Point, Cell> = HashMap::new();
    // let total = (rowmax as usize + 1) * (colmax as usize + 1);
    // let step = (total / 1000).max(1); // 0.1% increments
    //
    // let mut index = 0usize;
    for corner in &corners {
        grid.insert(
            Point {
                column: corner.column,
                row: corner.row,
            },
            Cell::Corner,
        );
    }
    for border in &border {
        grid.insert(
            Point {
                column: border.column,
                row: border.row,
            },
            Cell::Border,
        );
    }
    // print_grid(&grid);
    //=========================
    // println!("Time to add padding!");
    let grid = add_padding(grid);
    // print_grid(&grid);

    // println!("{} Rectangles: {rectangles:?}", rectangles.len());
    'next_rect: for (i, rectangle) in rectangles.into_iter().enumerate().skip(skips) {
        {
            //top line
            let row = rectangle.row_min;
            for column in rectangle.col_min..=rectangle.col_max {
                if matches!(grid.get(&Point { column, row }), Some(Cell::Padding)) {
                    // println!("\tTouching padding at row {row} column {column}");
                    continue 'next_rect;
                }
            }
        }

        {
            //bottom line
            let row = rectangle.row_max;
            for column in rectangle.col_min..=rectangle.col_max {
                if matches!(grid.get(&Point { column, row }), Some(Cell::Padding)) {
                    // println!("\tTouching padding at row {row} column {column}");
                    continue 'next_rect;
                }
            }
        }

        {
            //left line err here
            let column = rectangle.col_min;
            for row in rectangle.row_min..=rectangle.row_max {
                if matches!(grid.get(&Point { column, row }), Some(Cell::Padding)) {
                    // println!("\tTouching padding at row {column} column {row}");
                    continue 'next_rect;
                }
            }
        }
        {
            //right line
            let column = rectangle.col_max;
            for row in rectangle.row_min..=rectangle.row_max {
                if matches!(grid.get(&Point { column, row }), Some(Cell::Padding)) {
                    // println!("\tTouching padding at row {column} column {row}");
                    continue 'next_rect;
                }
            }
        }
        println!(
            "Found rectangle  number {}: {rectangle:?} area {}",
            i,
            rectangle.area()
        );
        // print_grid_with_rectangle(&rectangle, &grid);
        return rectangle.area();
    }

    todo!()
}

fn add_padding(mut grid: HashMap<Point, Cell>) -> HashMap<Point, Cell> {
    let mut padding_start = Point { column: 0, row: 0 };
    let mut alternating = false;
    loop {
        if grid.contains_key(&Point {
            column: padding_start.column,
            row: padding_start.row,
        }) {
            break;
        }

        if alternating {
            padding_start.row += 1;
        } else {
            padding_start.column += 1;
        }
        alternating = !alternating;
    }
    println!("Border or corner start: {padding_start:?}");
    // print_grid_with_marker(padding_start.row, padding_start.column, &grid);

    // let up = grid.get(&Point{column: padding_start.column, row: padding_start.row -1});
    // let right = grid.get(&Point{column: padding_start.column + 1, row: padding_start.row});
    // let down = grid.get(&Point{column: padding_start.column , row: padding_start.row + 1});
    // let left = grid.get(&Point{column: padding_start.column - 1, row: padding_start.row});
    // println!("Left: {left:?}, up: {up:?}, right: {right:?}, down: {down:?}");
    // print_around_point(padding_start.row, padding_start.column, &grid);

    let mut points_visited = HashSet::new();
    // walk around the figure clockwise, marking the left hand side as padding
    let mut position = padding_start;
    let mut walk_direction = East;
    while !points_visited.contains(&position) {
        // print_grid_with_marker(position.row, position.column, &grid);
        let north_point = Point {
            column: position.column,
            row: position.row - 1,
        };
        let north_cell = grid.get(&north_point).copied();
        let east_point = Point {
            column: position.column + 1,
            row: position.row,
        };
        let east_cell = grid.get(&east_point).copied();
        let south_point = Point {
            column: position.column,
            row: position.row + 1,
        };
        let south_cell = grid.get(&south_point).copied();
        let west_point = Point {
            column: position.column - 1,
            row: position.row,
        };
        let west_cell = grid.get(&west_point).copied();

        let mut directions = vec![
            (north_point, North, north_cell),
            (east_point, East, east_cell),
            (south_point, South, south_cell),
            (west_point, West, west_cell),
        ];

        match walk_direction {
            East => {
                if north_cell.is_none() {
                    grid.insert(north_point, Cell::Padding);
                }
            }
            North => {
                if west_cell.is_none() {
                    grid.insert(west_point, Cell::Padding);
                }
            }
            South => {
                if east_cell.is_none() {
                    grid.insert(east_point, Cell::Padding);
                }
            }
            West => {
                if south_cell.is_none() {
                    grid.insert(south_point, Cell::Padding);
                }
            }
        }
        //remove direction  we came from
        directions.retain(|(_, direction, _)| *direction != walk_direction.opposite());
        directions.retain(|(_, _, cell)| matches!(cell, Some(Cell::Border | Cell::Corner)));
        // println!("Walking: {walk_direction:?}, alternatives: {directions:?}");
        assert_eq!(directions.len(), 1);
        points_visited.insert(position);
        position = directions[0].0;
        walk_direction = directions[0].1;
    }

    grid
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    East,
    North,
    South,
    West,
}

impl Direction {
    const fn opposite(self) -> Self {
        match self {
            East => West,
            North => South,
            South => North,
            West => East,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2025/day9.txt");
        assert_eq!(part1(input), 4765757080);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2025/day9.txt");
        assert_eq!(part2(input), 1498673376);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#,
        );

        assert_eq!(result, 50)
    }

    #[test]
    fn part2_provided_example() {
        let result = solve_part2(
            r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#,
            0,
        );

        assert_eq!(result, 24)
    }
}
