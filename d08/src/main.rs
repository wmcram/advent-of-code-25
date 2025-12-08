use std::collections::{BinaryHeap, HashMap, HashSet};

fn dist(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    let (x1, y1, z1) = a;
    let (x2, y2, z2) = b;
    (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)
}

// Basic DFS connected components routine
fn connected_components(num_nodes: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(u, v) in edges {
        adj.entry(u).or_default().push(v);
        adj.entry(v).or_default().push(u);
    }

    let mut visited = HashSet::new();
    let mut comps = Vec::new();
    let mut stk = Vec::new();

    for start in 0..num_nodes {
        if visited.contains(&start) {
            continue;
        }

        // fresh
        stk.push(start);
        visited.insert(start);
        let mut comp = Vec::new();

        while let Some(u) = stk.pop() {
            comp.push(u);
            if let Some(nbs) = adj.get(&u) {
                for &nb in nbs {
                    if !visited.contains(&nb) {
                        visited.insert(nb);
                        stk.push(nb);
                    }
                }
            }
        }

        comps.push(comp);
    }

    comps
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let coords: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|l| {
            let v: Vec<i64> = l.split(",").map(|n| n.parse().unwrap()).collect();
            match &v[..] {
                [x, y, z] => (*x, *y, *z),
                _ => unimplemented!(),
            }
        })
        .collect();

    let mut dists: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let d = dist(coords[i], coords[j]);
            dists.push((d, i, j));
        }
    }
    dists.sort();
    let edges: Vec<_> = dists.into_iter().map(|(_, i, j)| (i, j)).collect();

    for i in 0..edges.len() {
        let components = connected_components(coords.len(), &edges[0..=i]);
        if components.len() == 1 {
            let (j, k) = edges[i];
            let res = coords[j].0 * coords[k].0;
            println!("{res}");
            return;
        }
    }
}
