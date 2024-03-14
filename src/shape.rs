#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
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
    // adjacency_list: Vec<(Position, Vec<Position>)>,
    pub data: [[LetterData; 5]; 5],
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

        // [2/3][d/t][£]c
        let mut letter = ' ';
        let (mut word_multiplier, mut letter_multiplier, mut has_gem): (u8, u8, bool) =
            (1, 1, false);
        for (y, line) in graph.lines().enumerate() {
            for (x, letter_group) in line.split(' ').enumerate() {
                println!("{y}, {x}: {}", letter_group);
                for c in letter_group.chars() {
                    if let Some(multiplier) = c.to_digit(10) {
                        word_multiplier = multiplier as u8;
                    } else if c.is_uppercase() {
                        letter_multiplier = if c == 'D' { 2 } else { 3 };
                    } else if c == '£' {
                        has_gem = true;
                    } else {
                        letter = c;
                    }
                }
                // Apply
                characters[y][x] = letter;
                // word_multiplier[y][x] = w_multiplier;
                // letter_multiplier[y][x] = l_multiplier;
                // has_gem[y][x] = l_has_gem;
                data[y][x] = LetterData {
                    word_multiplier,
                    letter_multiplier,
                    has_gem,
                };

                // Reset
                letter = ' ';
                (word_multiplier, letter_multiplier, has_gem) = (1, 1, false);
            }
        }

        Graph {
            characters,
            // values: [[0; 5]; 5],
            // adjacency_list: vec![(Position { x: 1, y: 1 }, vec![Position { x: 1, y: 1 }])],
            data,
        }
    }
}
