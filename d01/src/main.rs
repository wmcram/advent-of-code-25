use std::fs;

fn main() {
    let mut res = 0;
    let mut cur = 50;
    for line in fs::read_to_string("input").unwrap().lines() {
        let mut chars = line.chars();
        let mut sign = 1;
        match chars.next().unwrap() {
            'L' => sign = -1,
            _ => (),
        }

        let num: i32 = chars.as_str().parse().unwrap();
        let prev = cur;
        cur += num * sign;
        if prev * cur < 0 || cur == 0 {
            res += 1;
        }
        res += i32::abs(cur / 100);
        cur %= 100;
    }

    println!("{res}");
}
