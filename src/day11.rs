use std::collections::HashMap;

fn number_of_digits(n: usize) -> u32 {
    if n == 0 {
        return 1;
    }
    n.ilog10() + 1
}

fn split_in_two(n: usize) -> (usize, usize) {
    let d = 10usize.pow(number_of_digits(n) / 2);
    (n / d, n % d)
}

pub fn read_stones(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

pub fn count_stone(
    stone: usize,
    iterations: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if iterations == 0 {
        return 1;
    }

    if let Some(&val) = memo.get(&(stone, iterations)) {
        return val;
    }

    let result = if stone == 0 {
        count_stone(1, iterations - 1, memo)
    } else if number_of_digits(stone) % 2 == 0 {
        let (lhs, rhs) = split_in_two(stone);
        count_stone(lhs, iterations - 1, memo) + count_stone(rhs, iterations - 1, memo)
    } else {
        count_stone(stone * 2024, iterations - 1, memo)
    };

    memo.insert((stone, iterations), result);

    result
}

pub fn blink_n(stones: &[usize], iterations: usize) -> usize {
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
    stones
        .iter()
        .map(|&s| count_stone(s, iterations, &mut memo))
        .sum()
}

#[test]
fn day() {
    let stones = read_stones("125 17");
    assert_eq!(blink_n(&stones, 1), 3);
    assert_eq!(blink_n(&stones, 2), 4);
    assert_eq!(blink_n(&stones, 3), 5);
    assert_eq!(blink_n(&stones, 4), 9);
    assert_eq!(blink_n(&stones, 6), 22);
    assert_eq!(blink_n(&stones, 25), 55312);

    let stones2 = read_stones(&std::fs::read_to_string("input/day11").unwrap());
    assert_eq!(blink_n(&stones2, 25), 200446);
    assert_eq!(blink_n(&stones2, 75), 238317474993392);
}
