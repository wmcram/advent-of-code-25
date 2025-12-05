fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut res = 0;

    for line in input.lines() {
        let digs: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut on: Vec<char> = vec![];
        let mut start = 0;
        for i in 0..12 {
            let mut best = -1;
            let end = digs.len() - 12 + i + 1;
            for j in start..end {
                let c = digs[j];
                if c as i32 > best {
                    best = c as i32;
                    start = j + 1;
                }
            }
            on.push(best.to_string().chars().next().unwrap());
        }

        let amt: u64 = on.iter().collect::<String>().parse().unwrap();
        res += amt;
    }

    println!("{res}");
}
