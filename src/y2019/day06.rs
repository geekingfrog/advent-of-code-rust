use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn answer1() {
    let edges = read_edges("data/2019/day06.txt");
    let orbits = build_tree(edges);
    let result = count_orbits(&orbits, &"COM".to_string(), 0);
    println!("{}", result);
}

pub fn answer2() {
    let edges = read_edges("data/2019/day06.txt");
    let graph = build_graph(edges);
    let paths = shortest_path(&graph, &"YOU".to_string());
    println!("{}", paths.get("SAN").unwrap() - 2);
}

#[allow(dead_code)]
fn read_edges(file_path: &str) -> Vec<(String, String)> {
    let f = File::open(file_path).unwrap();
    let fd = BufReader::new(f);
    let re: Regex = Regex::new(r"^([^\)]+)\)(.+)$").unwrap();
    fd.lines()
        .map(|l| {
            let raw = l.unwrap().clone();
            let caps = re.captures(&raw).unwrap();
            (
                caps.get(1).unwrap().as_str().to_string(),
                caps.get(2).unwrap().as_str().to_string(),
            )
        })
        .collect()
}

fn build_tree(edges: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    for (center, orb) in edges.iter() {
        let orbits = m.entry(center.to_string()).or_insert(vec![]);
        orbits.push(orb.to_string());
    }
    m
}

// same as build_tree, but also add the edges child -> parent
fn build_graph(edges: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let all_edges: Vec<(String, String)> = edges
        .iter()
        .flat_map(|(src, dest)| {
            let a: String = src.to_string();
            let b: String = dest.to_string();
            vec![(a.clone(), b.clone()), (b, a)]
        })
        .collect();
    build_tree(all_edges)
}

fn count_orbits(orbits: &HashMap<String, Vec<String>>, source: &String, current: u32) -> u32 {
    match orbits.get(source) {
        None => current,
        Some(others) => {
            let x: u32 = others
                .iter()
                .map(|o| count_orbits(orbits, &o, current + 1))
                .sum();
            x + current
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: String,
}

// implement Ord so that the heap becomes a min-heap instead of a max-heap
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
fn shortest_path(graph: &HashMap<String, Vec<String>>, source: &String) -> HashMap<String, usize> {
    let mut heap = BinaryHeap::new();
    let mut dists: HashMap<String, usize> = HashMap::new();
    dists.insert(source.clone(), 0);
    heap.push(State {
        cost: 0,
        node: source.to_string(),
    });

    while let Some(State { cost, node }) = heap.pop() {
        if let Some(d) = dists.get(&node) {
            if &cost > d {
                continue;
            }
        }

        // for each node reachable from the current one, see if there is a shorter
        // path going through this node
        for neighbour in graph.get(&node).unwrap() {
            let next = State {
                cost: cost + 1,
                node: neighbour.to_string(),
            };
            if next.cost < *dists.get(&next.node).unwrap_or(&std::usize::MAX) {
                dists.insert(next.node.to_string(), next.cost);
                heap.push(next);
            }
        }
    }

    dists
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_orbits_simple() {
        let edges = read_edges("data/2019/day06_test.txt");
        let orbits = build_tree(edges);
        assert_eq!(count_orbits(&orbits, &"COM".to_string(), 0), 42);
    }

    #[test]
    fn test_orbit_transfer() {
        let edges = read_edges("data/2019/day06_test2.txt");
        let graph = build_graph(edges);
        let paths = shortest_path(&graph, &"YOU".to_string());
        assert_eq!(paths.get("SAN"), Some(&6));
    }
}
