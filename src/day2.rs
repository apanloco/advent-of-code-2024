#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::Result;

fn is_line_safe(line: &str) -> bool {
    let nums: Vec<&str> = line.split_whitespace().collect();
    let nums: Vec<usize> = nums.into_iter().map(|n| n.parse().unwrap()).collect();
    let mut diffs = Vec::new();
    for w in nums.windows(2) {
        let diff = w[0] as i64 - w[1] as i64;
        if !diffs.is_empty() {
            if diff == 0 {
                return false;
            }
            if diff > 0 && !diffs.iter().all(|d| d > &0) {
                return false;
            }
            if diff < 0 && !diffs.iter().all(|d| d < &0) {
                return false;
            }
        }
        if !(diff.abs() >= 1 && diff.abs() <= 3) {
            return false;
        }
        diffs.push(diff);
    }
    true
}

fn count_safe(input: &str) -> usize {
    input.lines().filter(|l| is_line_safe(l.trim())).count()
}

fn is_line_safe_dampener(line: &str) -> bool {
    let nums: Vec<&str> = line.split_whitespace().collect();
    for i in 0..nums.len() {
        let mut nums = nums.clone();
        nums.remove(i);
        let new_line = nums.join(" ");
        if is_line_safe(&new_line) {
            return true;
        }
    }
    false
}

fn count_safe_dampener(input: &str) -> usize {
    input
        .lines()
        .filter(|l| is_line_safe_dampener(l.trim()))
        .count()
}

#[test]
fn day() -> Result<()> {
    let data1 = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9    "#
        .trim();
    assert!(data1.starts_with("7"));
    assert!(data1.ends_with("9"));
    let data2 = std::fs::read_to_string("input/day2")?;

    assert_eq!(count_safe(data1), 2);
    assert_eq!(count_safe(&data2), 660);
    assert_eq!(count_safe_dampener(data1), 4);
    assert_eq!(count_safe_dampener(&data2), 689);

    Ok(())
}
