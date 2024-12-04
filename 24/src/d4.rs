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
    input.iter().enumerate().fold(0, |acc, (i, v)| {
        acc + v.iter().enumerate().fold(0, |acc, (j, &c)| {
            acc + if c == 'X' { seek_xmas(&input, i, j) } else { 0 }
        })
    })
}

fn seek_cross_mas(input: &[Vec<char>], i: usize, j: usize) -> i32 {
    // Cannot be bordering
    if i == 0 || j == 0 || i == input.len() - 1 || j == input[0].len() - 1 {
        return 0;
    }
    let expected = match (input[i + 1][j + 1], input[i + 1][j - 1]) {
        ('S', 'S') => [(i - 1, j - 1, 'M'), (i - 1, j + 1, 'M')],
        ('S', 'M') => [(i - 1, j - 1, 'M'), (i - 1, j + 1, 'S')],
        ('M', 'S') => [(i - 1, j - 1, 'S'), (i - 1, j + 1, 'M')],
        ('M', 'M') => [(i - 1, j - 1, 'S'), (i - 1, j + 1, 'S')],
        _ => return 0,
    };
    expected
        .into_iter()
        .all(|(i_ex, j_ex, char_ex)| input[i_ex][j_ex] == char_ex)
        .into()
}

pub fn run_2(input: String) -> i32 {
    let input: Vec<Vec<char>> = input
        .split("\n")
        .map(|s| s.chars().collect::<Vec<char>>())
        .filter(|v| !v.is_empty())
        .collect();
    input.iter().enumerate().fold(0, |acc, (i, v)| {
        acc + v.iter().enumerate().fold(0, |acc, (j, &c)| {
            acc + if c == 'A' {
                seek_cross_mas(&input, i, j)
            } else {
                0
            }
        })
    })
}
