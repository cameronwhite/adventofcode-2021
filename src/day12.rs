use std::collections::{HashMap, HashSet};

type AdjList = HashMap<String, Vec<String>>;

#[aoc_generator(day12)]
fn parse(input: &str) -> AdjList {
    let mut adj_list = AdjList::new();

    for s in input.lines() {
        let (a, b) = s.split_once("-").unwrap();

        adj_list
            .entry(a.to_string())
            .or_default()
            .push(b.to_string());
        adj_list
            .entry(b.to_string())
            .or_default()
            .push(a.to_string());
    }

    adj_list
}

fn dfs(vtx: &String, adj_list: &AdjList, visited: &mut HashSet<String>) -> usize {
    if visited.contains(vtx) {
        return 0;
    }

    if vtx == "end" {
        return 1;
    }

    let is_lower = vtx.chars().next().unwrap().is_lowercase();
    if is_lower {
        visited.insert(vtx.clone());
    }

    let mut sum = 0;
    for dst in &adj_list[vtx] {
        sum += dfs(dst, adj_list, visited);
    }

    if is_lower {
        visited.remove(vtx);
    }

    sum
}

#[aoc(day12, part1)]
fn solve_part1(adj_list: &AdjList) -> usize {
    dfs(&"start".to_string(), adj_list, &mut HashSet::new())
}

fn dfs2(
    vtx: &String,
    adj_list: &AdjList,
    visited: &mut HashSet<String>,
    dup: Option<String>,
) -> usize {
    if vtx == "end" {
        return 1;
    }

    let is_lower = vtx.chars().next().unwrap().is_lowercase();
    let mut dup = dup.clone();
    if is_lower && visited.contains(vtx) {
        if vtx != "start" && dup.is_none() {
            dup = Some(vtx.clone());
        } else {
            return 0;
        }
    }

    visited.insert(vtx.clone());

    let mut sum = 0;
    for dst in &adj_list[vtx] {
        sum += dfs2(dst, adj_list, visited, dup.clone());
    }

    if dup.is_none() || dup.unwrap() != *vtx {
        visited.remove(vtx);
    }

    sum
}

#[aoc(day12, part2)]
fn solve_part2(adj_list: &AdjList) -> usize {
    dfs2(&"start".to_string(), adj_list, &mut HashSet::new(), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        assert_eq!(solve_part1(&parse(input)), 10);
        assert_eq!(solve_part2(&parse(input)), 36);
    }
}
