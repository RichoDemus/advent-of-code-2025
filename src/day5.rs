use aoc_runner_derive::aoc;
use std::collections::VecDeque;
use tracing::info;

type Start = usize;
type End = usize;

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    let (ranges, ingredients) = {
        let mut iter = input.split("\n\n");
        let ranges = iter.next().unwrap();
        let ingredients = iter.next().unwrap();
        (ranges, ingredients)
    };

    let ranges = parse_ranges(ranges);
    // info!("Ranges: {ranges:?}");

    let ingredients = ingredients
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // info!("Ingredients: {ingredients:?}");

    ingredients
        .iter()
        .filter(|ingredient| {
            for range in &ranges {
                let Range(start, end) = range;
                if *ingredient >= start && *ingredient <= end {
                    return true;
                }
            }
            false
        })
        .count()
}

fn parse_ranges(ranges: &str) -> Vec<Range> {
    ranges
        .lines()
        .map(|range| {
            let ranges = range.split('-').collect::<Vec<_>>();
            Range(ranges[0].parse().unwrap(), ranges[1].parse().unwrap())
        })
        .collect()
}

#[derive(Debug, Copy, Clone)]
struct Range(Start, End);

#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
    let (ranges, _ingredients) = {
        let mut iter = input.split("\n\n");
        let ranges = iter.next().unwrap();
        let ingredients = iter.next().unwrap();
        (ranges, ingredients)
    };

    let mut ranges = parse_ranges(ranges);

    ranges.sort_by_key(|range| range.0);
    info!("Ranges: {ranges:?}");
    let mut ranges = VecDeque::from(ranges);

    let mut new_ranges = VecDeque::new();

    loop {
        info!("Loop: {ranges:?}");
        let mut did_change = false;
        'next: loop {
            info!("\t{new_ranges:?} | {ranges:?}");
            if ranges.len() < 2 {
                let mut temp = ranges.clone();
                ranges.clone_from(&new_ranges);
                ranges.append(&mut temp);
                new_ranges.clear();
                break 'next;
            }
            let left = ranges.pop_front().unwrap();
            let right = ranges.front().unwrap();

            if left.1 < right.0 {
                // no overlap
                new_ranges.push_back(left);
                continue;
            }
            let new_range = Range(left.0, left.1.max(right.1));
            let _ = ranges.pop_front();
            ranges.push_front(new_range);
            did_change = true;
        }
        if !did_change {
            break;
        }
    }
    info!("Done: {ranges:?}");
    let mut sum = 0;
    for Range(start, end) in &ranges {
        sum += end - start + 1;
    }
    sum
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
        let input = include_str!("../input/2025/day5.txt");
        assert_eq!(part1(input), 567);
    }

    #[test]
    fn verify_part2() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(LevelFilter::OFF)
            .try_init();
        let input = include_str!("../input/2025/day5.txt");
        assert_eq!(part2(input), 354149806372909);
    }

    #[test]
    fn part1_provided_example() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();
        let result = part1(
            r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#,
        );

        assert_eq!(result, 3)
    }

    #[test]
    fn part2_provided_example() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();
        let result = part2(
            r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#,
        );

        assert_eq!(result, 14)
    }
}
