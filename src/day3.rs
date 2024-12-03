#![allow(unused_imports)]
#![allow(dead_code)]

use regex::Regex;

enum Mode {
    Basic,
    OnOff,
}

fn is_on(input: &str, pos: usize) -> bool {
    let data = &input[0..pos];
    match (data.rfind("do()"), data.rfind("don't()")) {
        (None, None) => true,
        (_, None) => true,
        (None, _) => false,
        (Some(doo), Some(dont)) => doo > dont,
    }
}

fn compute(input: &str, mode: Mode) -> u64 {
    let re = Regex::new("mul\\((?<lhs>\\d+),(?<rhs>\\d+)\\)").unwrap();
    let mut sum = 0;
    let caps = re.captures_iter(input);
    for cap in caps {
        if let Mode::OnOff = mode {
            if !is_on(input, cap.get(0).unwrap().start()) {
                continue;
            }
        }
        let lhs: u64 = cap["lhs"].parse().unwrap();
        let rhs: u64 = cap["rhs"].parse().unwrap();
        sum += lhs * rhs;
    }
    sum
}

#[test]
fn day() {
    let data1 = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    let data2 = std::fs::read_to_string("input/day3").unwrap();

    assert_eq!(compute(data1, Mode::Basic), 161);
    assert_eq!(compute(&data2, Mode::Basic), 155955228);

    assert_eq!(compute(data1, Mode::OnOff), 48);
    assert_eq!(compute(&data2, Mode::OnOff), 100189366);
}
