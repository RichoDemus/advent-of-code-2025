use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    const fn distance_to(&self, other: &Self) -> i64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).abs()
    }
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    solve_part1(input, 1000)
}

fn solve_part1(input: &str, connections: usize) -> usize {
    let junction_boxes: Vec<JunctionBox> = input
        .lines()
        .map(|line| {
            let numbers: Vec<i64> = line.split(',').map(|c| c.parse().unwrap()).collect();
            JunctionBox {
                x: numbers[0],
                y: numbers[1],
                z: numbers[2],
            }
        })
        .collect();

    let shortest_distance = junction_boxes
        .iter()
        .permutations(2)
        .map(|junctions| (junctions[0].distance_to(junctions[1]), junctions))
        .sorted_by_key(|(distance, _)| *distance)
        .map(|(distance, boxes)| (distance, *boxes[0], *boxes[1]))
        .dedup_by(|(_distance1, a1, b1), (_distance2, a2, b2)| {
            (a1 == a2 && b1 == b2) || (a1 == b2 && a2 == b1)
        })
        .take(connections)
        .collect::<Vec<_>>();

    let mut lowest_circuit = 0usize;
    let mut circuits: HashMap<JunctionBox, usize> = HashMap::new();
    for (_distance, left, right) in &shortest_distance {
        let left_option = circuits.get(left).copied();
        let right_option = circuits.get(right).copied();
        match (left_option, right_option) {
            (Some(left_circuit), Some(right_circuit)) => {
                //two different circuits, need to merge them
                circuits
                    .iter_mut()
                    .filter(|(_box, circuit)| **circuit == right_circuit)
                    .for_each(|(_box, circuit)| *circuit = left_circuit);
            }
            (Some(circuit), None) => {
                circuits.insert(*right, circuit);
            }
            (None, Some(circuit)) => {
                circuits.insert(*left, circuit);
            }
            (None, None) => {
                lowest_circuit += 1;
                circuits.insert(*left, lowest_circuit);
                circuits.insert(*right, lowest_circuit);
            }
        }
    }

    let grouped = circuits
        .iter()
        .sorted_by_key(|(_key, circuit)| *circuit)
        .chunk_by(|(_key, circuit)| *circuit);
    let mut result = 1;
    for (_circuit, boxes) in grouped
        .into_iter()
        .map(|(circuit, boxes)| (*circuit, boxes.collect::<Vec<_>>()))
        .sorted_by_key(|(_circuit, boxes)| boxes.len())
        .rev()
        .take(3)
    {
        result *= boxes.len();
    }

    result
}

#[aoc(day8, part2)]
fn part2(input: &str) -> i64 {
    let mut junction_boxes: VecDeque<JunctionBox> = input
        .lines()
        .map(|line| {
            let numbers: Vec<i64> = line.split(',').map(|c| c.parse().unwrap()).collect();
            JunctionBox {
                x: numbers[0],
                y: numbers[1],
                z: numbers[2],
            }
        })
        .collect();

    // find first pair
    let shortest_distance = junction_boxes
        .iter()
        .permutations(2)
        .map(|junctions| (junctions[0].distance_to(junctions[1]), junctions))
        .sorted_by_key(|(distance, _)| *distance)
        .map(|(distance, boxes)| (distance, *boxes[0], *boxes[1]))
        .dedup_by(|(_distance1, a1, b1), (_distance2, a2, b2)| {
            (a1 == a2 && b1 == b2) || (a1 == b2 && a2 == b1)
        })
        .take(1)
        .collect::<Vec<_>>();
    let box1 = shortest_distance[0].1;
    let box2 = shortest_distance[0].2;
    junction_boxes.retain_mut(|b| *b != box1 && *b != box2);
    let mut circuit = vec![box1, box2];
    let mut x1 = 0;
    let mut x2 = 0;
    loop {
        if junction_boxes.is_empty() {
            break;
        }
        // let start = vec![circuit.first().cloned().unwrap(), circuit.last().cloned().unwrap()].iter();
        let start = circuit.iter();
        let (_distance, box_in_circuit, box_to_add) = start
            .flat_map(|box_in_circuit| {
                junction_boxes
                    .iter()
                    .map(|box_outside_circuit| (*box_in_circuit, *box_outside_circuit))
            })
            .map(|(box_in_circuit, box_outside_circuit)| {
                (
                    box_in_circuit.distance_to(&box_outside_circuit),
                    box_in_circuit,
                    box_outside_circuit,
                )
            })
            .min_by_key(|(distance, _, _)| *distance)
            .unwrap();
        x1 = box_to_add.x;
        x2 = box_in_circuit.x;
        junction_boxes.retain(|b| *b != box_to_add);

        circuit.push(box_to_add);
    }

    x1 * x2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2025/day8.txt");
        assert_eq!(part1(input), 96672);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2025/day8.txt");
        assert_eq!(part2(input), 22517595);
    }

    #[test]
    fn part1_provided_example() {
        let result = solve_part1(
            r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#,
            10,
        );

        assert_eq!(result, 40)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#,
        );

        assert_eq!(result, 25272)
    }
}
