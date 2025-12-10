use z3::{Optimize, SatResult, ast::Int};

struct Machine {
    target: u64,
    buttons: Vec<u64>,
    button_indices: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}

fn minimal_xor(target: u64, buttons: &[u64]) -> u64 {
    let n = buttons.len();
    let mut best = u64::MAX;

    for mask in 0..(1u64 << n) {
        let mut x = 0;
        let mut presses = 0;
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                x ^= buttons[i];
                presses += 1;
            }
        }
        if x == target {
            best = best.min(presses);
        }
    }

    best
}

// Find the minimal number of button presses for this machine to reach target joltage using z3
fn minimal_joltage(m: &Machine) -> u64 {
    let opt = Optimize::new();
    let total = Int::fresh_const("total");

    // number of presses for each button, must be >= 0
    let presses: Vec<Int> = (0..m.buttons.len())
        .map(|i| Int::fresh_const(&format!("b{i}")))
        .collect();
    presses.iter().for_each(|b| opt.assert(&b.ge(0)));

    // for each target joltage, the number of presses that contribute to this joltage must equal
    // the target
    for (i, &target) in m.joltages.iter().enumerate() {
        let mut terms = Vec::new();
        for (j, button) in m.button_indices.iter().enumerate() {
            if button.contains(&i) {
                terms.push(presses[j].clone());
            }
        }

        let tot = Int::add(&terms.iter().collect::<Vec<&Int>>());
        opt.assert(&tot.eq(Int::from_u64(target)));
    }

    // define total num presses and minimize
    opt.assert(&total.eq(Int::add(&presses)));
    opt.minimize(&total);

    match opt.check(&[]) {
        SatResult::Sat => opt
            .get_model()
            .unwrap()
            .eval(&total, true)
            .and_then(|v| v.as_u64())
            .unwrap(),
        _ => unimplemented!(),
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let machines: Vec<Machine> = input
        .lines()
        .map(|l| {
            let mut it = l.split(" ").peekable();
            let lights = it.next().unwrap();
            let lights = &lights[1..lights.len() - 1];
            let mut target = 0u64;
            for (i, c) in lights.chars().enumerate() {
                if c == '#' {
                    target |= 1 << i;
                }
            }

            let mut buttons: Vec<u64> = vec![];
            let mut button_indices: Vec<Vec<usize>> = vec![];
            while let Some(s) = it.next() {
                if it.peek().is_none() {
                    let joltages: Vec<u64> = (&s[1..s.len() - 1])
                        .split(",")
                        .map(|t| t.parse().unwrap())
                        .collect();
                    return Machine {
                        target,
                        buttons,
                        button_indices,
                        joltages,
                    };
                } else {
                    let button: Vec<u32> = (&s[1..s.len() - 1])
                        .split(",")
                        .map(|t| t.parse().unwrap())
                        .collect();
                    let amt: u64 = button.iter().fold(0, |acc, v| acc | (1u64 << v));
                    let indices: Vec<usize> = button.iter().map(|v| *v as usize).collect();
                    buttons.push(amt);
                    button_indices.push(indices);
                }
            }
            unreachable!()
        })
        .collect();

    let res: u64 = machines.iter().map(|m| minimal_joltage(m)).sum();
    println!("{res}");
}
