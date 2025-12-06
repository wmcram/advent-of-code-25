fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut grid: Vec<Vec<bool>> = vec![];
    for l in input.lines() {
        let mut cur = vec![];
        for c in l.chars() {
            match c {
                '@' => cur.push(true),
                '.' => cur.push(false),
                _ => unimplemented!(),
            }
        }
        grid.push(cur);
    }

    let (m, n) = (grid.len(), grid[0].len());
    let mut res = 0;

    loop {
        let mut acc = 0;
        for i in 0..m {
            for j in 0..n {
                if !grid[i][j] {
                    continue;
                }
                let mut cnt = 0;
                for (dx, dy) in [
                    (0, 1),
                    (1, 0),
                    (-1, 0),
                    (0, -1),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1),
                ] {
                    let (cx, cy) = (i as i32 + dx, j as i32 + dy);
                    if 0 <= cx
                        && cx < m as i32
                        && 0 <= cy
                        && cy < n as i32
                        && grid[cx as usize][cy as usize]
                    {
                        cnt += 1;
                    }
                }

                if cnt < 4 {
                    acc += 1;
                    grid[i][j] = false;
                }
            }
        }
        if acc == 0 {
            break;
        }
        res += acc;
    }

    println!("{res}");
}
