#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow::Context;
use anyhow::Result;

fn get_sorted_vecs(input: &str) -> Result<(Vec<usize>, Vec<usize>)> {
    let left_right: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let left = parts.next().unwrap().parse().unwrap();
            let right = parts.next().unwrap().parse().unwrap();
            (left, right)
        })
        .collect();
    let (mut left, mut right): (Vec<_>, Vec<_>) = left_right.into_iter().unzip();
    left.sort();
    right.sort();
    Ok((left, right))
}

fn compute_sum(input: &str) -> Result<usize> {
    let (left, right) = get_sorted_vecs(input)?;
    let sum: usize = left.iter().zip(&right).map(|(l, r)| l.abs_diff(*r)).sum();
    Ok(sum)
}

fn compute_similarity(input: &str) -> Result<usize> {
    let (left, right) = get_sorted_vecs(input)?;
    let similarity: usize = left
        .into_iter()
        .map(|n1| n1 * right.iter().filter(|&&n| n == n1).count())
        .sum();
    Ok(similarity)
}

#[test]
fn day() -> Result<()> {
    let data1 = r#"
3   4
4   3
2   5
1   3
3   9
3   3
    "#
    .trim();
    assert!(data1.starts_with("3"));
    assert!(data1.ends_with("3"));
    assert_eq!(compute_sum(data1)?, 11);

    let data2 = std::fs::read_to_string("input/day1")?;
    assert_eq!(compute_sum(&data2)?, 1151792);

    assert_eq!(compute_similarity(data1)?, 31);
    assert_eq!(compute_similarity(&data2)?, 21790168);

    Ok(())
}
