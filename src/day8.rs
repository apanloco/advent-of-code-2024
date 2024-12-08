use itertools::Itertools;

#[derive(Default)]
pub struct Game {
    pub antennas: Vec<Antenna>,
    pub width: isize,
    pub height: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug)]
pub struct Antenna {
    pub pos: Vec2,
    pub freq: char,
}

impl Vec2 {
    pub fn new(x: isize, y: isize) -> Vec2 {
        Vec2 { x, y }
    }
    pub fn delta(&self, rhs: Vec2) -> Vec2 {
        Vec2::new(rhs.x - self.x, rhs.y - self.y)
    }
    pub fn translate(&self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Game {
    pub fn inside_boundaries(&self, pos: Vec2) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width && pos.y < self.height
    }
}

pub fn read_game(input: &str) -> Game {
    let mut game = Game::default();
    for (y, row) in input.lines().enumerate() {
        game.height = (y + 1) as isize;
        for (x, c) in row.chars().enumerate() {
            game.width = (x + 1) as isize;
            if c != '.' {
                game.antennas.push(Antenna {
                    pos: Vec2::new(x as isize, y as isize),
                    freq: c,
                });
            }
        }
    }
    game
}

pub fn generate_antinodes_for_antenna_pair(
    game: &Game,
    lhs: &Antenna,
    rhs: &Antenna,
    resonance: bool,
) -> Vec<Vec2> {
    let delta = lhs.pos.delta(rhs.pos);
    let mut antinodes = vec![lhs.pos.translate(delta)];
    if resonance {
        antinodes.extend([lhs.pos, rhs.pos]);
        let mut pos = antinodes[0];
        while game.inside_boundaries(pos) {
            pos = pos.translate(delta);
            antinodes.push(pos);
        }
    }
    antinodes
}

pub fn get_antinodes(game: &Game, resonance: bool) -> Vec<Vec2> {
    game.antennas
        .iter()
        .permutations(2)
        .filter(|pair| pair[0].freq == pair[1].freq)
        .flat_map(|pair| generate_antinodes_for_antenna_pair(game, pair[0], pair[1], resonance))
        .filter(|pos| game.inside_boundaries(*pos))
        .unique()
        .collect()
}

#[test]
fn day() {
    let data1 = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#
    .trim();
    let game = read_game(data1);
    assert_eq!(game.antennas.len(), 7);
    assert_eq!(game.width, 12);
    assert_eq!(game.height, 12);
    let antinodes = get_antinodes(&game, false);
    assert_eq!(antinodes.len(), 14);
    let antinodes = get_antinodes(&game, true);
    assert_eq!(antinodes.len(), 34);
    let data2 = std::fs::read_to_string("input/day8").unwrap();
    let game2 = read_game(&data2);
    let antinodes2 = get_antinodes(&game2, false);
    assert_eq!(antinodes2.iter().unique().count(), 261);
    let antinodes2 = get_antinodes(&game2, true);
    assert_eq!(antinodes2.iter().unique().count(), 898);
}
