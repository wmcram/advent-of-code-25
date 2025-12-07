use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut lines = input.lines();
    let mut beams: HashMap<usize, u64> = HashMap::new();

    let first_line = lines.next().unwrap();
    for i in 0..first_line.len() {
        beams.insert(i, 0);
    }
    let start_pos = first_line.find("S").unwrap();
    beams.insert(start_pos, 1);
    for l in lines {
        for (i, c) in l.chars().enumerate() {
            if c == '^' && beams.contains_key(&i) {
                *beams.get_mut(&(i + 1)).unwrap() += beams[&i];
                *beams.get_mut(&(i - 1)).unwrap() += beams[&i];
                *beams.get_mut(&i).unwrap() = 0;
            }
        }
    }

    let res: u64 = beams.into_values().sum();

    println!("{res}");
}
