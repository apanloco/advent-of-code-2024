use itertools::Itertools;

#[derive(Default)]
pub struct Map {
    heights: Vec<isize>,
    width: isize,
    height: isize,
}

const ALL_DIRECTIONS: &[(isize, isize)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];

impl Map {
    pub fn new(input: &str) -> Map {
        let mut map = Map::default();
        for line in input.trim().lines() {
            let line = line.trim();
            map.width = line.len() as isize;
            map.height += 1;
            map.heights
                .extend(line.chars().map(|c| c.to_digit(10).unwrap() as isize))
        }
        map
    }

    fn at(&self, pos: (isize, isize)) -> isize {
        let index = pos.1 * self.width + pos.0;
        self.heights[index as usize]
    }

    fn index_to_pos(&self, index: usize) -> (isize, isize) {
        let index = index as isize;
        let y = index / self.width;
        let x = index - (y * self.width);
        (x, y)
    }

    pub fn within_bounds(&self, pos: (isize, isize)) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.width && pos.1 < self.height
    }

    fn heads(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.heights
            .iter()
            .enumerate()
            .filter(|(_i, &h)| h == 0)
            .map(|(i, _h)| self.index_to_pos(i))
    }

    pub fn sum_scores(&self) -> usize {
        self.heads()
            .map(|head| {
                let mut found: Vec<(isize, isize)> = Vec::new();
                self.count_trails_recursively(head, -1, &mut found);
                found.into_iter().unique().count()
            })
            .sum()
    }

    pub fn sum_ratings(&self) -> usize {
        let mut _found: Vec<(isize, isize)> = Vec::new();
        self.heads()
            .map(|head| self.count_trails_recursively(head, -1, &mut _found))
            .sum()
    }

    pub fn count_trails_recursively(
        &self,
        pos: (isize, isize),
        last_height: isize,
        found_nines: &mut Vec<(isize, isize)>,
    ) -> usize {
        if !self.within_bounds(pos) || self.at(pos) != last_height + 1 {
            return 0;
        }
        if self.at(pos) == 9 {
            found_nines.push(pos);
            return 1;
        }
        ALL_DIRECTIONS
            .iter()
            .map(|direction| {
                self.count_trails_recursively(
                    (pos.0 + direction.0, pos.1 + direction.1),
                    self.at(pos),
                    found_nines,
                )
            })
            .sum()
    }
}

#[test]
fn day() {
    let data1 = r#"
    89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732
    "#;
    let map1 = Map::new(data1);
    assert_eq!(map1.heads().count(), 9);
    assert_eq!(*map1.heads().collect::<Vec<_>>().first().unwrap(), (2, 0));
    assert_eq!(*map1.heads().collect::<Vec<_>>().last().unwrap(), (1, 7));
    assert_eq!(map1.at((0, 0)), 8);
    assert_eq!(map1.at((7, 7)), 2);
    assert_eq!(map1.sum_scores(), 36);
    assert_eq!(map1.sum_ratings(), 81);

    let data2 = std::fs::read_to_string("input/day10").unwrap();
    let map2 = Map::new(&data2);
    assert_eq!(map2.sum_scores(), 717);
    assert_eq!(map2.sum_ratings(), 1686);
}
