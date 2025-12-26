use aoc_runner_derive::aoc;
use itertools::Itertools;

fn parse(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let divider = input.rfind("\n\n").unwrap();
    let (shapes, regions) = input.split_at(divider);

    let shapes = shapes.trim().split("\n\n").map(parse_shape).collect_vec();
    let regions = regions.trim().lines().map(parse_region).collect_vec();

    (shapes, regions)
}

fn parse_shape(block: &str) -> Shape {
    Shape {
        shape: block
            .lines()
            .skip(1)
            .map(|line| line.chars().map(|c| c == '#').collect_vec())
            .collect_vec(),
    }
}

fn parse_region(line: &str) -> Region {
    let split = line.split(':').collect_vec();
    let dimensions: Vec<usize> = split[0]
        .split('x')
        .map(|d| d.parse().unwrap())
        .collect_vec();
    let width = dimensions[0];
    let length = dimensions[1];

    let shapes: Vec<usize> = split[1]
        .trim()
        .split_ascii_whitespace()
        .map(|i| i.parse().unwrap())
        .collect_vec();

    Region {
        width,
        length,
        shapes,
    }
}

#[derive(Debug, Clone)]
struct Shape {
    shape: Vec<Vec<bool>>,
}

impl Shape {
    fn size(&self) -> usize {
        self.shape.iter().flatten().filter(|b| **b).count()
    }
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    length: usize,
    shapes: Vec<usize>,
}

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    let (shapes, regions) = parse(input);

    println!("shapes: {shapes:?}");
    println!("regions: {regions:?}");

    solve_day1_naive(shapes.as_slice(), regions.as_slice())
}

fn solve_day1_naive(shapes: &[Shape], regions: &[Region]) -> usize {
    regions
        .iter()
        .filter(|region| do_all_shapes_fit_naive(shapes, region))
        .count()
}

fn do_all_shapes_fit_naive(shapes: &[Shape], region: &Region) -> bool {
    let mut cells = 0;
    for (shape, amount) in region.shapes.iter().enumerate() {
        let shape = &shapes[shape];
        cells += shape.size() * amount;
    }
    cells <= region.length * region.width
}

// #[aoc(day12, part2)]
// fn part2(input: &str) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2025/day12.txt");
        assert_eq!(part1(input), 448);
    }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2025/day12.txt");
    //     assert_eq!(part2(input), 0);
    // }

    //     #[test]
    //     fn part1_provided_example() {
    //         let result = part1(r#"0:
    // ###
    // ##.
    // ##.
    //
    // 1:
    // ###
    // ##.
    // .##
    //
    // 2:
    // .##
    // ###
    // ##.
    //
    // 3:
    // ##.
    // ###
    // ##.
    //
    // 4:
    // ###
    // #..
    // ###
    //
    // 5:
    // ###
    // .#.
    // ###
    //
    // 4x4: 0 0 0 0 2 0
    // 12x5: 1 0 1 0 2 2
    // 12x5: 1 0 1 0 3 2"#);
    //
    //         assert_eq!(result, 2)
    //     }

    // #[test]
    // fn part2_provided_example() {
    //     let result = part2(
    //         r#""#,
    //     );
    //
    //     assert_eq!(result, 0)
    // }
}
