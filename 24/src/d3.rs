pub fn run_1(input: String) -> i32 {
    let candidates = input.match_indices("mul(");
    let input: Vec<char> = input.chars().collect();
    candidates.into_iter().fold(0, |acc, (i, _)| {
        let mut i = i + 4; // Skip over `mul(`
        let saved = i;
        let mut n1 = 0;
        while input[i].is_numeric() {
            n1 *= 10;
            n1 += input[i].to_digit(10).unwrap() as i32;
            i += 1;
        }
        // no number  || no comma
        if i == saved || input[i] != ',' {
            return acc;
        }
        i += 1;
        let saved = i;
        let mut n2 = 0;
        while input[i].is_numeric() {
            n2 *= 10;
            n2 += input[i].to_digit(10).unwrap() as i32;
            i += 1;
        }
        // no second number
        if i == saved || input[i] != ')' {
            return acc;
        }
        acc + n1 * n2
    })
}
