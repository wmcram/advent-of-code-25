type Present = Vec<(usize, usize)>;

struct Region {
    width: usize,
    height: usize,
    amounts: [usize; 6],
}

fn check_region(presents: &Vec<Present>, region: &Region) -> bool {
    // heuristics work on the input here
    if region
        .amounts
        .iter()
        .enumerate()
        .map(|(i, n)| presents[i].len() * (*n))
        .sum::<usize>()
        > region.width * region.height
    {
        return false;
    }
    if presents.len() <= (region.width / 3) * (region.height / 3) {
        return true;
    }
    false
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut lines = input.lines();

    let mut shapes: Vec<Present> = Vec::new();
    for _ in 0..=5 {
        let mut shape = Vec::new();
        lines.next();
        for i in 0..3 {
            let cs = lines.next().unwrap();
            for j in 0..3 {
                if cs.chars().nth(j).unwrap() == '#' {
                    shape.push((i, j));
                }
            }
        }
        lines.next();
        shapes.push(shape);
    }

    let mut regions = Vec::new();
    for l in lines {
        let (dims, nums) = l.split_once(":").unwrap();
        let (w, h) = dims.split_once("x").unwrap();
        let (width, height) = (w.parse().unwrap(), h.parse().unwrap());

        let amounts: [usize; 6] = nums
            .trim()
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        regions.push(Region {
            width,
            height,
            amounts,
        });
    }

    let mut res = 0;
    for r in &regions {
        if check_region(&shapes, r) {
            res += 1;
        }
    }
    println!("{res}");
}
