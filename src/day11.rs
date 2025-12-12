use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let servers = input
        .lines()
        .map(|line| {
            let split1 = line.split(':').collect_vec();
            let name = split1[0].to_string();
            let connections = split1[1].trim();
            let connections = connections
                .split_ascii_whitespace()
                .map(ToString::to_string)
                .collect_vec();
            Server { name, connections }
        })
        .collect_vec();

    // println!("servers: {servers:?}");
    calculate_paths(&servers, "you", "out", &[])
}

#[derive(Debug)]
struct Server {
    name: String,
    connections: Vec<String>,
}

fn calculate_paths(servers: &[Server], start: &str, end: &str, avoid: &[&str]) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let servers: HashMap<String, Vec<String>> = servers
        .iter()
        .map(|server| (server.name.clone(), server.connections.clone()))
        .collect();
    let mut solutions = HashSet::new();

    queue.push_front({
        let mut x = VecDeque::new();
        x.push_front(start.to_string());
        x
    });

    while let Some(path) = queue.pop_front() {
        println!("{start}->{end} Handling: {path:?}");
        if path.front().unwrap() == end {
            solutions.insert(path);
            continue;
        }

        let node = path.front().unwrap().clone();
        for next in servers.get(&node).unwrap_or(&vec![]) {
            if path.contains(next) {
                continue;
            }
            if avoid.contains(&&**next) {
                continue;
            }
            let mut path_clone = path.clone();
            path_clone.push_front(next.clone());
            if visited.contains(&path_clone) {
                continue;
            }
            visited.insert(path_clone.clone());
            queue.push_front(path_clone);
        }
    }

    solutions.len()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    let servers = input
        .lines()
        .map(|line| {
            let split1 = line.split(':').collect_vec();
            let name = split1[0].to_string();
            let connections = split1[1].trim();
            let connections = connections
                .split_ascii_whitespace()
                .map(ToString::to_string)
                .collect_vec();
            Server { name, connections }
        })
        .collect_vec();

    let servers2: HashMap<String, Vec<String>> = servers
        .iter()
        .map(|server| (server.name.clone(), server.connections.clone()))
        .collect();

    println!("svr => fft");
    let svr_to_fft = calc_paths_rec(&servers2, "svr".to_string(), "fft", &mut HashMap::new());
    println!("\t{svr_to_fft} paths");
    println!("fft => dac");
    let fft_to_dac = calc_paths_rec(&servers2, "fft".to_string(), "dac", &mut HashMap::new());
    println!("\t{fft_to_dac} paths");
    println!("dav => out");
    let dac_to_out = calc_paths_rec(&servers2, "dac".to_string(), "out", &mut HashMap::new());
    println!("\t{dac_to_out} paths");

    // evil assumption, dac can never be before fft
    svr_to_fft * fft_to_dac * dac_to_out
}

fn calc_paths_rec(
    servers: &HashMap<String, Vec<String>>,
    node: String,
    goal: &str,
    memoization: &mut HashMap<String, usize>,
) -> usize {
    if let Some(paths) = memoization.get(&node) {
        return *paths;
    }
    if node == goal {
        return 1;
    }

    let mut paths = 0;
    for next in servers.get(&node).unwrap_or(&vec![]) {
        let p = calc_paths_rec(servers, next.clone(), goal, memoization);
        paths += p;
    }
    memoization.insert(node, paths);
    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2025/day11.txt");
        assert_eq!(part1(input), 649);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2025/day11.txt");
        assert_eq!(part2(input), 458948453421420);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#,
        );

        assert_eq!(result, 5)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#,
        );

        assert_eq!(result, 2)
    }
}
