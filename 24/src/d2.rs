pub fn run_1(input: String) -> i32 {
    let reports = input
        .split('\n')
        .map(|s| {
            s.split_whitespace()
                .map(|str_num| str_num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let mut safe = 0;
    for r in reports {
        if r.is_empty() {
            continue;
        }
        let valid = if r[0] < r[1] {
            |a, b| a < b
        } else {
            |a, b| a > b
        };
        for i in 1..r.len() {
            let a = r[i - 1];
            let b = r[i];
            let diff = (a - b).abs();
            if !valid(a, b) || !(1..=3).contains(&diff) {
                safe -= 1;
                break;
            }
        }
        safe += 1;
    }
    safe
}
