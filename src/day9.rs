#![allow(clippy::needless_range_loop)]

#[derive(Debug)]
pub enum Block {
    File(usize),
    Free,
}

pub fn read_disk(input: &str) -> Vec<Block> {
    let mut map = Vec::new();
    for (index, c) in input.trim().chars().enumerate() {
        let c: usize = c.to_digit(10).unwrap() as usize;
        if index % 2 == 0 {
            for _ in 0..c {
                map.push(Block::File(index / 2));
            }
        } else {
            for _ in 0..c {
                map.push(Block::Free);
            }
        }
    }
    map
}

pub fn find_empty(disk: &[Block], size: usize) -> Option<usize> {
    let mut start = None;
    let mut end;
    for (index, b) in disk.iter().enumerate() {
        if matches!(b, Block::Free) {
            if start.is_none() {
                start = Some(index);
            }
            end = Some(index);
        } else {
            start = None;
            end = None;
        }
        if let (Some(s), Some(e)) = (start, end) {
            if e - s + 1 == size {
                return start;
            }
        }
    }
    None
}

pub fn find_block(disk: &[Block]) -> Option<usize> {
    disk.iter().rposition(|b| matches!(b, Block::File(_)))
}

pub fn find_file(disk: &[Block], id: usize) -> Option<(usize, usize)> {
    let start = disk.iter().position(|b| {
        if let Block::File(file_id) = b {
            return *file_id == id;
        }
        false
    });
    let start = start?;
    let end = disk[start..].iter().position(|b| {
        if let Block::File(file_id) = b {
            return *file_id != id;
        }
        true
    });
    let size = match end {
        Some(i) => i,
        None => disk.len() - start,
    };
    Some((start, size))
}

pub fn defragment_simple(mut disk: Vec<Block>) -> Vec<Block> {
    loop {
        let block_pos = find_block(&disk).unwrap();
        let empty_pos = find_empty(&disk, 1).unwrap();
        if empty_pos >= block_pos {
            break;
        }
        disk.swap(empty_pos, block_pos);
    }
    disk
}

pub fn defragment_smart(mut disk: Vec<Block>) -> Vec<Block> {
    let highest_id = *disk
        .iter()
        .filter_map(|b| match b {
            Block::File(id) => Some(id),
            Block::Free => None,
        })
        .max()
        .unwrap();
    for id in (0..=highest_id).rev() {
        let (file_pos, file_size) = find_file(&disk, id).unwrap();
        let empty_pos = find_empty(&disk, file_size);
        let Some(empty_pos) = empty_pos else {
            continue;
        };
        if empty_pos >= file_pos {
            continue;
        }
        for i in empty_pos..(empty_pos + file_size) {
            disk[i] = Block::File(id);
        }
        for i in file_pos..(file_pos + file_size) {
            disk[i] = Block::Free;
        }
    }
    disk
}

pub fn checksum(disk: &[Block]) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(index, block)| match block {
            Block::File(id) => Some(id * index),
            Block::Free => None,
        })
        .sum()
}

#[test]
fn day() {
    let data1 = r#"2333133121414131402"#;
    let disk = read_disk(data1);
    assert_eq!(
        disk.len(),
        "00...111...2...333.44.5555.6666.777.888899".len()
    );

    assert_eq!(find_file(&disk, 9), Some((40, 2)));
    assert_eq!(find_file(&disk, 8), Some((36, 4)));
    assert_eq!(find_file(&disk, 7), Some((32, 3)));

    let disk = defragment_simple(read_disk(data1));
    assert_eq!(checksum(&disk), 1928);

    let disk = defragment_smart(read_disk(data1));
    assert_eq!(checksum(&disk), 2858);

    let data2 = std::fs::read_to_string("input/day9").unwrap();
    let disk2 = read_disk(&data2);
    let disk2 = defragment_simple(disk2);
    assert_eq!(checksum(&disk2), 6301895872542);

    let disk2 = read_disk(&data2);
    let disk2 = defragment_smart(disk2);
    assert_eq!(checksum(&disk2), 6323761685944);
}
