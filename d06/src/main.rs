fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut res = 0;
    let mut nums: Vec<u64> = Vec::new();
    let mut op = None;

    // iter over cols
    for x in 0..width {
        let mut col_chars = Vec::new();
        for y in 0..height {
            col_chars.push(grid[y].get(x).copied().unwrap());
        }

        // separator case
        if col_chars.iter().all(|c| c.is_whitespace()) {
            if !nums.is_empty() {
                let chunk_res = match op.unwrap() {
                    '+' => nums.iter().sum::<u64>(),
                    '*' => nums.iter().product::<u64>(),
                    _ => unimplemented!(),
                };
                res += chunk_res;
                nums.clear();
                op = None;
            }
            continue;
        }

        let op_char = *col_chars.last().unwrap();
        if op_char == '+' || op_char == '*' {
            op = Some(op_char);
        }

        let digits: String = col_chars[0..height - 1]
            .iter()
            .filter(|c| c.is_digit(10))
            .collect();

        if let Ok(num) = digits.parse::<u64>() {
            nums.push(num);
        }
    }

    if !nums.is_empty() {
        let chunk_res = match op.unwrap() {
            '+' => nums.iter().sum::<u64>(),
            '*' => nums.iter().product::<u64>(),
            _ => unimplemented!(),
        };
        res += chunk_res;
    }

    println!("{res}");
}
