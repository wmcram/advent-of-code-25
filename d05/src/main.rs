fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut ranges: Vec<(u64, u64)> = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (a, b) = l.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    ranges.sort_unstable();

    let mut merged: Vec<(u64, u64)> = vec![];
    for (s, e) in ranges {
        match merged.last_mut() {
            Some((_, me)) if s <= *me + 1 => *me = (*me).max(e),
            _ => merged.push((s, e)),
        }
    }

    let res: u64 = merged.iter().map(|(s, e)| e - s + 1).sum();
    println!("{res}");
}
