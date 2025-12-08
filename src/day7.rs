use aoc_runner_derive::aoc;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let mut splits = 0;
    let mut beams: HashSet<usize> = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            let start_index = line.find('S').unwrap();
            beams.insert(start_index);
            continue;
        }

        let mut new_beams: HashSet<usize> = HashSet::new();
        for beam in &beams {
            match line.chars().nth(*beam).unwrap() {
                '.' => {
                    new_beams.insert(*beam);
                }
                '^' => {
                    splits += 1;
                    new_beams.insert(*beam - 1);
                    new_beams.insert(*beam + 1);
                }
                c => panic!("Unhandled char {c}"),
            }
        }
        beams.clear();
        beams.clone_from(&new_beams);
    }
    splits
}

type Cache = Arc<Mutex<HashMap<(Vec<usize>, usize), usize>>>;
#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let cache: Cache = Arc::new(Mutex::new(HashMap::new()));
    let parsed = input
        .lines()
        .skip(1)
        .map(|line| {
            line.chars()
                .map(|c| if c == '^' { Cell::Splitter } else { Cell::Dot })
                .collect::<Vec<_>>()
        })
        .collect::<VecDeque<_>>();

    let mut beams: HashSet<usize> = HashSet::new();
    let start_index = input.lines().next().unwrap().find('S').unwrap();
    beams.insert(start_index);

    calc_number_of_paths(cache, &beams, parsed, 0)
}

#[derive(Copy, Clone)]
enum Cell {
    Dot,
    Splitter,
}

fn calc_number_of_paths(
    cache: Cache,
    previous_beams: &HashSet<usize>,
    lines: VecDeque<Vec<Cell>>,
    index: usize,
) -> usize {
    let mut garuanteed_beams = HashSet::new();
    let mut potential_beams = HashSet::new();

    if index >= lines.len() {
        return 1;
    }
    let line = lines.get(index).unwrap().clone();
    for beam in previous_beams {
        match line[*beam] {
            Cell::Dot => {
                garuanteed_beams.insert(*beam);
            }
            Cell::Splitter => {
                potential_beams.insert(beam - 1);
                potential_beams.insert(beam + 1);
            }
        }
    }
    let mut paths = 0;
    if potential_beams.is_empty() {
        paths += calc_number_of_paths(cache, &garuanteed_beams, lines, index + 1);
    } else {
        for beam in potential_beams {
            let mut beams = HashSet::new();
            beams.clone_from(&garuanteed_beams);
            beams.insert(beam);
            let value = cache
                .lock()
                .unwrap()
                .get(&(beams.iter().copied().collect::<Vec<_>>(), index))
                .copied();
            if let Some(cached) = value {
                paths += cached;
            } else {
                let path = calc_number_of_paths(cache.clone(), &beams, lines.clone(), index + 1);
                cache
                    .lock()
                    .unwrap()
                    .insert((beams.iter().copied().collect::<Vec<_>>(), index), path);
                paths += path;
            }
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::filter::LevelFilter;
    #[test]
    fn verify_part1() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(LevelFilter::OFF)
            .try_init();
        let input = include_str!("../input/2025/day7.txt");
        assert_eq!(part1(input), 1539);
    }

    #[test]
    fn verify_part2() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(LevelFilter::OFF)
            .try_init();
        let input = include_str!("../input/2025/day7.txt");
        assert_eq!(part2(input), 6479180385864);
    }

    #[test]
    fn part1_provided_example() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();
        let result = part1(
            r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#,
        );

        assert_eq!(result, 21)
    }

    #[test]
    fn part2_provided_example() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();
        let result = part2(
            r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#,
        );

        assert_eq!(result, 40)
    }
}
