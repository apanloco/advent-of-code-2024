#![allow(dead_code)]
#![allow(clippy::ptr_arg)]

use std::collections::HashMap;

fn find_word_in_grid(
    grid: &Vec<Vec<char>>,
    word: &str,
    mut x: isize,
    mut y: isize,
    movement: (isize, isize),
) -> bool {
    for c in word.chars() {
        if x >= width(grid) || y >= height(grid) || x < 0 || y < 0 || char_at(grid, x, y) != c {
            return false;
        }
        x += movement.0;
        y += movement.1;
    }
    true
}

fn char_at(grid: &Vec<Vec<char>>, x: isize, y: isize) -> char {
    grid[y as usize][x as usize]
}

fn width(grid: &Vec<Vec<char>>) -> isize {
    grid[0].len() as isize
}

fn height(grid: &Vec<Vec<char>>) -> isize {
    grid.len() as isize
}

fn to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

const ALL_DIRECTIONS: &[(isize, isize)] = &[
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

const DIAGONAL_DIRECTIONS: &[(isize, isize)] = &[(1, -1), (1, 1), (-1, 1), (-1, -1)];

fn count(input: &str, word: &str) -> usize {
    let grid = to_grid(input);
    let mut count = 0_usize;
    for y in 0..height(&grid) {
        for x in 0..width(&grid) {
            for movement in ALL_DIRECTIONS {
                count += find_word_in_grid(&grid, word, x, y, *movement) as usize;
            }
        }
    }
    count
}

fn count_cross(input: &str, word: &str) -> usize {
    let grid = to_grid(input);
    let mut positions = HashMap::new();
    for y in 0..height(&grid) {
        for x in 0..width(&grid) {
            for movement in DIAGONAL_DIRECTIONS {
                if find_word_in_grid(&grid, word, x, y, *movement) {
                    *positions
                        .entry((x + movement.0, y + movement.1))
                        .or_insert(0) += 1;
                }
            }
        }
    }
    positions.iter().filter(|&(_, &c)| c == 2).count()
}

#[test]
fn day() {
    let data1 = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
    "#
    .trim();
    let data2 = std::fs::read_to_string("input/day4").unwrap();
    assert!(find_word_in_grid(&to_grid(data1), "XMAS", 4, 0, (1, 1)));
    assert!(find_word_in_grid(&to_grid(data1), "XMAS", 5, 9, (1, 0)));
    assert_eq!(count(data1, "XMAS"), 18);
    assert_eq!(count(&data2, "XMAS"), 2344);
    assert_eq!(count_cross(data1, "MAS"), 9);
    assert_eq!(count_cross(&data2, "MAS"), 1815);
}
