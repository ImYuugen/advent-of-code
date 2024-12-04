fn seek_xmas(input: &[Vec<char>], i: usize, j: usize) -> i32 {
    let mas = ['M', 'A', 'S'];
    let (i, j) = (i as i64, j as i64);
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut total = 0;
    // Bourrinage incoming
    for d in directions {
        // No need to check for 0 as it will overlap way beyond len
        let (far_i, far_j) = ((d.0 * 3 + i) as usize, (d.1 * 3 + j) as usize);
        if far_i >= input.len() || far_j >= input[0].len() {
            continue;
        }

        if (1..=3).all(|step| {
            input[(i + d.0 * step) as usize][(j + d.1 * step) as usize] == mas[(step - 1) as usize]
        }) {
            total += 1;
        }
    }
    total
}

pub fn run_1(input: String) -> i32 {
    // Input is square so it be ok :3
    let input: Vec<Vec<char>> = input
        .split("\n")
        .map(|s| s.chars().collect::<Vec<char>>())
        .filter(|v| !v.is_empty())
        .collect();
    let mut res = 0;
    for (i, v) in input.iter().enumerate() {
        for (j, &c) in v.iter().enumerate() {
            if c == 'X' {
                res += seek_xmas(&input, i, j);
            }
        }
    }
    res
}

pub fn run_2(input: String) -> i32 {
    0
}
