use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
pub struct Game {
    map: Vec<Vec<char>>,
    x: isize,
    y: isize,
    direction: Direction,
}

#[derive(PartialEq, Debug)]
pub enum SimulationResult {
    Done(usize),
    Loop,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    pub fn to_movement(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

impl Game {
    pub fn width(&self) -> isize {
        self.map[0].len() as isize
    }
    pub fn height(&self) -> isize {
        self.map.len() as isize
    }
    pub fn size(&self) -> usize {
        self.map[0].len() * self.map.len()
    }
    pub fn within_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width() && y < self.height()
    }
    pub fn at(&self, x: isize, y: isize) -> char {
        self.map[y as usize][x as usize]
    }
    pub fn is_obstacle(&self, x: isize, y: isize) -> bool {
        if !self.within_bounds(x, y) {
            return false;
        }
        self.at(x, y) == '#'
    }
}

pub fn read_game_start(input: &str) -> Result<Game> {
    let mut pos: Option<(isize, isize)> = None;
    let mut map = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                pos = Some((x as isize, y as isize));
                row.push('.');
            } else {
                row.push(c);
            }
        }
        map.push(row);
    }
    let Some((x, y)) = pos else {
        anyhow::bail!("failed to find starting position");
    };
    Ok(Game {
        map,
        x,
        y,
        direction: Direction::Up,
    })
}

pub fn run_simulation(mut game: Game, extra_obstacle: Option<(isize, isize)>) -> SimulationResult {
    let mut visited_incl_dir = HashSet::with_capacity(game.size());
    let mut visited_positions = Vec::with_capacity(game.size());
    loop {
        if !game.within_bounds(game.x, game.y) {
            visited_positions.sort();
            visited_positions.dedup();
            return SimulationResult::Done(visited_positions.len());
        }
        let movement = game.direction.to_movement();
        let obstacle_x = game.x + movement.0;
        let obstacle_y = game.y + movement.1;
        if game.is_obstacle(obstacle_x, obstacle_y)
            || extra_obstacle == Some((obstacle_x, obstacle_y))
        {
            game.direction = game.direction.turn_right();
        } else {
            let new_pos = (game.x, game.y, movement.0, movement.1);
            if visited_incl_dir.contains(&new_pos) {
                return SimulationResult::Loop;
            }
            visited_incl_dir.insert(new_pos);
            visited_positions.push((game.x, game.y));
            let movement = game.direction.to_movement();
            game.x += movement.0;
            game.y += movement.1;
        }
    }
}

pub fn find_loops(game: Game) -> usize {
    (0..game.size())
        .into_par_iter()
        .map(|i| {
            (
                (i % game.width() as usize) as isize,
                (i / game.width() as usize) as isize,
            )
        })
        .filter(|&(x, y)| !game.is_obstacle(x, y))
        .filter(|&(x, y)| !(game.x == x && game.y == y))
        .filter(|&pos| {
            let result = run_simulation(game.clone(), Some(pos));
            matches!(result, SimulationResult::Loop)
        })
        .count()
}

#[test]
fn day() -> Result<()> {
    let data1 = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#
    .trim();
    let data2 = std::fs::read_to_string("input/day6")?;

    let game = read_game_start(data1)?;
    assert_eq!(game.x, 4);
    assert_eq!(game.y, 6);
    let result = run_simulation(game, None);
    assert_eq!(result, SimulationResult::Done(41));

    let game = read_game_start(&data2)?;
    let result = run_simulation(game, None);
    assert_eq!(result, SimulationResult::Done(4977));

    let game = read_game_start(data1)?;
    let result = run_simulation(game, Some((3, 6)));
    assert_eq!(result, SimulationResult::Loop);

    let game = read_game_start(data1)?;
    let result = run_simulation(game, Some((6, 7)));
    assert_eq!(result, SimulationResult::Loop);

    let game = read_game_start(data1)?;
    let result = run_simulation(game, Some((7, 7)));
    assert_eq!(result, SimulationResult::Loop);

    let game = read_game_start(data1)?;
    let loops = find_loops(game);
    assert_eq!(loops, 6);

    let game = read_game_start(&data2)?;
    let loops = find_loops(game);
    assert_eq!(loops, 1729);

    Ok(())
}
