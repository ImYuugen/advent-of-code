use std::collections::{HashMap, HashSet};

pub fn run_1(input: String) -> i32 {
    let rules = input
        .split('\n')
        .take_while(|s| !s.is_empty())
        .map(|p| {
            let ps: Vec<&str> = p.split('|').collect();
            (ps[0].parse().unwrap(), ps[1].parse().unwrap())
        })
        .collect::<Vec<(i32, i32)>>();
    let mut rules_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for rule in rules {
        rules_map
            .entry(rule.0)
            .and_modify(|v| {
                v.insert(rule.1);
            })
            .or_insert(HashSet::from([rule.1]));
    }
    let updates = input
        .split('\n')
        .skip_while(|s| !s.is_empty())
        .skip(1) // Newline
        .filter(|s| !s.is_empty())
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut res = 0;
    for update in updates {
        let mut counted = true;
        let mut encountered: HashSet<i32> = HashSet::new();
        for i in (0..update.len()).rev() {
            if let Some(rule) = rules_map.get(&update[i]) {
                if !encountered.is_subset(rule) {
                    // A rule wasn't met
                    counted = false;
                    break;
                }
            }
            encountered.insert(update[i]);
        }
        if counted {
            res += update[update.len() / 2];
        }
    }
    res
}

pub fn run_2(input: String) -> i32 {
    0
}
