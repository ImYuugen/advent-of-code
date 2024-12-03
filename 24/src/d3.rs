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

pub fn run_2(input: String) -> i32 {
    let input = input.as_str();
    let mut res = 0;
    let mut offset = 0;
    'toplevel: while offset < input.len() {
        while offset < input.len() && !input[offset..].starts_with("don't()") {
            if input[offset..].starts_with("mul(") {
                offset += 4;
                let Some(mut next_non_num) = input[offset..].find(|c: char| !c.is_numeric()) else {
                    // End of string
                    break 'toplevel;
                };
                next_non_num += offset;
                if let Ok(n1) = input[offset..next_non_num].parse::<i32>() {
                    offset = next_non_num + 1;
                    let Some(mut next_non_num) = input[offset..].find(|c: char| !c.is_numeric())
                    else {
                        // End of string
                        break 'toplevel;
                    };
                    next_non_num += offset;
                    // I hate this
                    if input[next_non_num..(next_non_num + 1)].eq(")") {
                        res += n1 * input[offset..next_non_num].parse::<i32>().unwrap_or(0);
                    }
                }
            }
            offset += 1;
        }
        if let Some(i) = input[offset..].find("do()") {
            offset += i + 4;
        }
    }
    res
}
