fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let ranges = input.trim().split(",");

    let mut res: u64 = 0;

    for range in ranges {
        let mut it = range.splitn(2, "-");
        let s: u64 = it.next().unwrap().parse().unwrap();
        let e: u64 = it.next().unwrap().parse().unwrap();
        for id in s..=e {
            let digs = id.to_string();
            for i in 1..=(digs.len() / 2) {
                if digs.len() % i != 0 {
                    continue;
                }
                let chunks: Vec<String> = digs
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(i)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect();
                let first = &chunks[0];
                if chunks.len() >= 2 && chunks.iter().all(|c| first == c) {
                    res += id;
                    break;
                }
            }
        }
    }

    println!("{res}");
}
