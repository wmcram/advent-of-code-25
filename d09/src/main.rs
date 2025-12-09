type Edge = ((u64, u64), (u64, u64));

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut coords: Vec<(u64, u64)> = input
        .lines()
        .map(|l| {
            let mut it = l.split(",");
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    let mut edges: Vec<Edge> = vec![];
    coords.push(coords[0]);
    for w in coords.windows(2) {
        edges.push((w[0], w[1]));
    }
    coords.pop();

    let mut res = 0;
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let ((mut x1, mut y1), (mut x2, mut y2)) = (coords[i], coords[j]);
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }
            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }

            let area = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);

            if edges.iter().all(|(s, e)| {
                let left = x2 <= s.0.min(e.0);
                let right = x1 >= s.0.max(e.0);
                let up = y2 <= s.1.min(e.1);
                let down = y1 >= s.1.max(e.1);
                left || right || up || down
            }) {
                res = res.max(area);
            }
        }
    }
    println!("{res}");
}
