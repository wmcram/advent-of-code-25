use std::collections::HashMap;

fn num_paths(from: &str, to: &str, adj: &HashMap<String, Vec<String>>) -> u64 {
    fn helper(
        from: &str,
        to: &str,
        adj: &HashMap<String, Vec<String>>,
        memo: &mut HashMap<String, u64>,
    ) -> u64 {
        let mut tot = 0;
        if adj.get(from).is_none() {
            return 0;
        }

        for nb in &adj[from] {
            if *nb == to {
                tot += 1;
            } else if memo.contains_key(nb) {
                tot += memo[nb];
            } else {
                let val = helper(nb, to, adj, memo);
                memo.insert(nb.clone(), val);
                tot += val;
            }
        }
        tot
    }

    let mut memo: HashMap<String, u64> = HashMap::new();
    helper(from, to, adj, &mut memo)
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();

    for l in input.lines() {
        let (from, to) = l.split_once(":").unwrap();
        let tos: Vec<String> = to.trim().split(" ").map(|s| s.to_string()).collect();
        adj.insert(from.to_string(), tos);
    }

    let res = num_paths("svr", "fft", &adj)
        * num_paths("fft", "dac", &adj)
        * num_paths("dac", "out", &adj);
    println!("{res}");
}
