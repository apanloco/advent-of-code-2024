use itertools::Itertools;
use std::collections::HashSet;

// <pos, direction>
type Fence = HashSet<((isize, isize), (isize, isize))>;

#[derive(Default)]
pub struct Map {
    pub plants: Vec<char>,
    pub width: isize,
    pub height: isize,
}

const ALL_DIRECTIONS: &[(isize, isize)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];

fn rotate_direction(direction: (isize, isize)) -> (isize, isize) {
    let index = ALL_DIRECTIONS.iter().position(|d| d == &direction).unwrap();
    if index == 0 {
        ALL_DIRECTIONS[ALL_DIRECTIONS.len() - 1]
    } else {
        ALL_DIRECTIONS[index - 1]
    }
}

fn find_fence_endpoint(
    (mut piece_pos, piece_direction): ((isize, isize), (isize, isize)),
    fence: &Fence,
) -> ((isize, isize), (isize, isize)) {
    let follow_direction = rotate_direction(piece_direction);
    loop {
        let new_pos = (
            piece_pos.0 + follow_direction.0,
            piece_pos.1 + follow_direction.1,
        );
        if !fence.contains(&(new_pos, piece_direction)) {
            break;
        }
        piece_pos = new_pos;
    }
    (piece_pos, piece_direction)
}

impl Map {
    pub fn new(input: &str) -> Map {
        let mut map = Map::default();
        for line in input.trim().lines() {
            let line = line.trim();
            map.width = line.len() as isize;
            map.height += 1;
            map.plants.extend(line.chars())
        }
        map
    }

    fn at(&self, pos: (isize, isize)) -> char {
        let index = pos.1 * self.width + pos.0;
        self.plants[index as usize]
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

    fn find_garden_recursively(
        &self,
        plant: char,
        pos: (isize, isize),
        positions: &mut HashSet<(isize, isize)>,
    ) {
        if !self.within_bounds(pos) {
            return;
        }
        if self.at(pos) != plant {
            return;
        }
        if positions.contains(&pos) {
            return;
        }
        positions.insert(pos);
        for direction in ALL_DIRECTIONS {
            self.find_garden_recursively(
                plant,
                (pos.0 + direction.0, pos.1 + direction.1),
                positions,
            );
        }
    }

    pub fn garden_from(&self, pos: (isize, isize)) -> Vec<(isize, isize)> {
        let mut positions = HashSet::new();
        let plant = self.at(pos);
        self.find_garden_recursively(plant, pos, &mut positions);
        positions.into_iter().unique().sorted().collect::<Vec<_>>()
    }

    pub fn gardens(&self) -> Vec<Vec<(isize, isize)>> {
        (0..self.plants.len())
            .map(|i| self.index_to_pos(i))
            .map(|pos| self.garden_from(pos))
            .unique()
            .collect()
    }

    pub fn fence(&self, garden: &Vec<(isize, isize)>) -> Fence {
        let mut fence = HashSet::new();
        for pos in garden {
            for direction in ALL_DIRECTIONS {
                let check = (pos.0 + direction.0, pos.1 + direction.1);
                fence.insert((check, *direction));
            }
        }
        for pos in garden {
            for direction in ALL_DIRECTIONS {
                fence.remove(&(*pos, *direction));
            }
        }
        fence
    }

    pub fn fence_cheap(
        &self,
        garden: &Vec<(isize, isize)>,
    ) -> HashSet<((isize, isize), (isize, isize))> {
        let fence = self.fence(garden);
        let sides = fence
            .iter()
            .map(|f| {
                println!("f: {:?}", f);
                let f2 = find_fence_endpoint(*f, &fence);
                println!("f2: {:?}", f2);
                f2
            })
            .unique()
            .collect();
        sides
    }

    pub fn price(&self) -> usize {
        self.gardens()
            .iter()
            .map(|g| g.len() * self.fence(g).len())
            .sum()
    }

    pub fn price_cheap(&self) -> usize {
        self.gardens()
            .iter()
            .map(|g| g.len() * self.fence_cheap(g).len())
            .sum()
    }
}

#[test]
fn day() {
    let input1 = r#"
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE
        "#
    .trim();
    let map1 = Map::new(input1);
    assert_eq!(map1.width, 10);
    assert_eq!(map1.height, 10);
    assert_eq!(map1.gardens().len(), 11);
    assert_eq!(map1.garden_from((0, 0)).len(), 12);
    assert_eq!(map1.fence(&map1.garden_from((0, 0))).len(), 18);
    assert_eq!(map1.garden_from((4, 0)).len(), 4);
    assert_eq!(map1.fence(&map1.garden_from((4, 0))).len(), 8);
    assert_eq!(map1.garden_from((8, 5)).len(), 13);
    assert_eq!(map1.fence(&map1.garden_from((9, 4))).len(), 18);
    assert_eq!(map1.price(), 1930);
    let map2 = Map::new(&std::fs::read_to_string("input/day12").unwrap());
    assert_eq!(map2.price(), 1464678);
    assert_eq!(map2.price_cheap(), 877492);
}
