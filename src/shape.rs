use std::collections::{HashMap, HashSet};

enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    const fn direction_to_relative_index(direction: &Direction) -> (isize, isize) {
        match direction {
            Direction::N => (-1, 0),
            Direction::NE => (-1, 1),
            Direction::E => (0, 1),
            Direction::SE => (1, 1),
            Direction::S => (1, 0),
            Direction::SW => (1, -1),
            Direction::W => (0, -1),
            Direction::NW => (-1, -1),
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct LetterData {
    // values: [[u8; 5]; 5],
    word_multiplier: u8,
    letter_multiplier: u8,
    has_gem: bool,
}

#[derive(Debug)]
pub struct Graph {
    pub characters: [[char; 5]; 5],
    // values: [[u8; 5]; 5],
    pub data: [[LetterData; 5]; 5],
    pub adjacency_list: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl Graph {
    pub fn new(graph: &str) -> Self {
        let mut characters = [[' '; 5]; 5];
        let letter_data = LetterData {
            word_multiplier: 1,
            letter_multiplier: 1,
            has_gem: false,
        };
        let mut data = [[letter_data; 5]; 5];
        let mut adjacency_list = HashMap::new();

        // [2/3][D/T][£]c
        let mut letter = ' ';
        let (mut word_multiplier, mut letter_multiplier, mut has_gem): (u8, u8, bool) =
            (1, 1, false);
        for (y, line) in graph.lines().enumerate() {
            for (x, letter_group) in line.split(' ').enumerate() {
                println!("{y}, {x}: {}", letter_group);
                for c in letter_group.chars() {
                    match c {
                        'a'..='z' => letter = c,
                        '0'..='9' => word_multiplier = c.to_digit(10).unwrap() as u8,
                        'D' => letter_multiplier = 2,
                        'T' => letter_multiplier = 3,
                        '£' => has_gem = true,
                        _ => println!("\x1b[31mIgnoring invalid character: \x1b[31;4m{}\x1b[0m", c),
                    }
                }

                // Apply
                characters[y][x] = letter;
                data[y][x] = LetterData {
                    word_multiplier,
                    letter_multiplier,
                    has_gem,
                };
                adjacency_list.insert(
                    (y, x),
                    get_neighbours(&characters, (y as isize, x as isize)),
                );

                // Reset
                letter = ' ';
                (word_multiplier, letter_multiplier, has_gem) = (1, 1, false);
            }
        }

        Graph {
            characters,
            data,
            adjacency_list,
        }
    }

    pub fn get_neighbours(&self, index: (usize, usize)) -> &Vec<(usize, usize)> {
        self.adjacency_list
            .get(&index)
            .expect("node should have neighbours!!!")
    }

    pub fn dfs_traverse(&self, start_index: Position) -> HashSet<Vec<Position>> {
        let word_paths = HashSet::new();

        println!("{:?}", start_index);
        println!("{}", std::mem::size_of_val(&start_index));
        println!("{}", std::mem::size_of_val(&(1_usize, 2_usize)));

        word_paths
    }
}

fn get_neighbours(shape_vec: &[[char; 5]; 5], index: (isize, isize)) -> Vec<(usize, usize)> {
    const DIRECTIONS: [Direction; 8] = [
        Direction::N,
        Direction::NE,
        Direction::E,
        Direction::SE,
        Direction::S,
        Direction::SW,
        Direction::W,
        Direction::NW,
    ];

    let mut neighbours: Vec<(usize, usize)> = Vec::with_capacity(8);
    for direction in DIRECTIONS {
        let dir = Direction::direction_to_relative_index(&direction);
        let new_index = (index.0 + dir.0, index.1 + dir.1);
        if (new_index.0 < 0 || new_index.0 >= shape_vec.len() as isize)
            || (new_index.1 < 0 || new_index.1 >= shape_vec[0].len() as isize)
        {
            continue;
        }

        let new_index: (usize, usize) = (new_index.0 as usize, new_index.1 as usize);
        neighbours.push(new_index);
    }

    neighbours
}
