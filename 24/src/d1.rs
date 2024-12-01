pub fn run_1(input: String) -> i32 {
    let mut lines: Vec<_> = input.split("\n").collect();
    lines.pop();
    let splits: Vec<_> = lines
        .into_iter()
        .map(|s| s.split("   ").collect::<Vec<_>>())
        .collect();
    let mut left = vec![];
    let mut right = vec![];
    for s in splits {
        left.push(s[0].parse::<i32>().unwrap());
        right.push(s[1].parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

use std::collections::HashMap;

pub fn run_2(input: String) -> i32 {
    let mut hist: HashMap<i32, i32> = HashMap::new();
    let mut lines: Vec<_> = input.split("\n").collect();
    lines.pop();
    let splits: Vec<_> = lines
        .into_iter()
        .map(|s| s.split("   ").collect::<Vec<_>>())
        .collect();
    let splits: Vec<_> = splits
        .into_iter()
        .map(|v| [v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap()])
        .collect();
    for n in splits.iter() {
        hist.entry(n[1]).and_modify(|e| *e += 1).or_insert(1);
    }
    splits
        .into_iter()
        .fold(0, |acc, v| acc + v[0] * hist.get(&v[0]).unwrap_or(&0))
}
