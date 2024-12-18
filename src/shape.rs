use crate::dictionary::Dictionary;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
pub struct LetterData {
    pure_value: u8,
    word_multiplier: u8,
    letter_multiplier: u8,
    has_gem: bool,
}

#[derive(Debug)]
pub struct Graph {
    pub characters: [[char; 5]; 5],
    pub data: [[LetterData; 5]; 5],
    pub adjacency_list: HashMap<(usize, usize), Box<[(usize, usize)]>>,
}

impl Graph {
    pub fn new(graph: &str) -> Self {
        let mut characters = [[' '; 5]; 5];
        let letter_data = LetterData {
            pure_value: 1,
            word_multiplier: 1,
            letter_multiplier: 1,
            has_gem: false,
        };
        let mut data = [[letter_data; 5]; 5];
        let mut adjacency_list = HashMap::with_capacity(25);

        // [2/3][D/T][£]c
        let mut letter = ' ';
        let (mut word_multiplier, mut letter_multiplier, mut has_gem) = (1, 1, false);
        for (y, line) in graph.lines().enumerate() {
            for (x, letter_group) in line.split(' ').enumerate() {
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
                let pure_value = evaluate(letter);
                data[y][x] = LetterData {
                    pure_value,
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

        // Convert to boxed_slices for more compact storage
        let mut adjacency_boxes = HashMap::new();
        for (k, v) in adjacency_list.into_iter() {
            adjacency_boxes.insert(k, v.into_boxed_slice());
        }

        Self {
            characters,
            data,
            adjacency_list: adjacency_boxes,
        }
    }

    fn get_stored_neighbours(&self, index: (usize, usize)) -> &[(usize, usize)] {
        self.adjacency_list
            .get(&index)
            .expect("node should have neighbours!!!")
    }

    pub fn dfs_traverse<'a>(
        &self, start_index: (usize, usize), swaps: u8, dictionary: &'a Dictionary,
    ) -> (Vec<Vec<(usize, usize)>>, Box<[&'a str]>) {
        let mut visited = HashSet::new();
        visited.insert(start_index);

        self.dfs(
            start_index,
            swaps,
            &mut visited,
            &mut vec![start_index],
            dictionary,
        )
    }

    // heuristic to explore most valuable first?
    fn dfs<'a>(
        &self, start_index: (usize, usize), swaps: u8, visited: &mut HashSet<(usize, usize)>,
        letter_indices: &mut Vec<(usize, usize)>, dictionary: &'a Dictionary,
    ) -> (Vec<Vec<(usize, usize)>>, Box<[&'a str]>) {
        //
        let mut word_paths = Vec::new();
        let mut swappable_words: Vec<&'a str> = Vec::new();

        let mut word = letter_indices
            .iter()
            .map(|&(x, y)| self.characters[x][y])
            .collect::<String>();

        let neighbours = self.get_stored_neighbours(start_index);
        for neighbour in neighbours {
            if visited.contains(neighbour) {
                continue;
            }
            // Visit, add potential prefix to word and vec.
            visited.insert(*neighbour);
            word.push(self.characters[neighbour.0][neighbour.1]);
            letter_indices.push(*neighbour);

            // If this new prefix is invalid, remove the letter's index from the vec, and the
            // character itself from the word and unvisit.
            // it to allow for other combinations with neighbouring letters
            // TODO: Cache invalid prefixes
            if word.len() > 5 {
                let len = word.len() as u8;
                let swap_search_space =
                    dictionary.get_values_from_range((len)..(len + swaps + 1));

                let swapped_words = swap_search_space
                    .into_par_iter()
                    .flatten()
                    .filter(|&&w| find_distance_betwixt_optimisedv2(w, &word) <= swaps)
                    .collect::<Vec<_>>();
                swappable_words.extend(swapped_words);
            }

            if !dictionary.is_valid_prefix(&word) {
                letter_indices.pop();
                word.pop();
                visited.remove(neighbour);
                // Invalid prefix

                continue;
            }

            // Check if it's valid when swaps are enabled
            if dictionary.is_valid_word(&word) {
                word_paths.push(letter_indices.clone());
            }

            let (valid, swapped) = self.dfs(*neighbour, swaps, visited, letter_indices, dictionary);
            word_paths.extend(valid);
            swappable_words.extend(swapped.iter());

            // Clean up before next neighbour
            letter_indices.pop();
            word.pop();
            visited.remove(neighbour);
        }

        (word_paths, swappable_words.into_boxed_slice())
    }

    // TODO: return the path with the greatest number of swaps remaining
    pub fn find_word_with_swaps(&self, target: &str, max_swaps: i8) -> Option<Vec<(usize, usize)>> {
        // Scenarios:
        // 1. current letter matches target's letter: continue
        // 2. current letter does not match the target's letter: swap - 1 if swap > 0

        for y in 0..5 {
            for x in 0..5 {
                let path = self._dfs(
                    target,
                    0,
                    (y, x),
                    vec![],
                    max_swaps,
                    &mut HashSet::from([(y, x)]),
                );

                if path.is_none() {
                    continue;
                } else {
                    return path;
                }
            }
        }
        None
    }

    fn _dfs(
        &self, target_word: &str, target_index: usize, current_index: (usize, usize),
        mut current_indices: Vec<(usize, usize)>, mut max_swaps: i8,
        visited: &mut HashSet<(usize, usize)>,
    ) -> Option<Vec<(usize, usize)>> {
        // Reached end of the word
        if current_indices.len() == target_word.len() {
            if max_swaps >= 0 {
                // let word = current_indices
                //     .iter()
                //     .map(|&(y, x)| self.characters[y][x])
                //     .collect::<String>();
                // println!("yes at end: {word}->{target_word} {max_swaps} {current_indices:?}");
                return Some(current_indices);
            }
            // println!("no: bounds reached or swaps exhausted: {max_swaps} {current_indices:?}");
            return None;
        }

        // Deduct a swap if the letters don't match
        let target_letter = target_word.chars().nth(target_index).unwrap();
        if target_letter != self.characters[current_index.0][current_index.1] {
            max_swaps -= 1;
        }
        current_indices.push(current_index);

        // Early exit: Exhausted swaps and wrong word
        if max_swaps < 0 {
            return None;
        }

        // Visit each unvisted neighbour for more permutations
        // E.g. re + (a | x | j | ...)
        let mut result = Vec::new();
        // TODO:  consider readding filter
        for neighbour in self.get_stored_neighbours(current_index) {
            if visited.contains(neighbour) {
                continue;
            }

            visited.insert(*neighbour);

            if let Some(res) = self._dfs(
                target_word,
                target_index + 1,
                *neighbour,
                current_indices.clone(),
                max_swaps,
                visited,
            ) {
                result.extend(res);
                break; // Remove to explore more
            }
            visited.remove(neighbour);
        }

        current_indices.pop();

        if result.is_empty() {
            return None;
        }

        Some(result)
    }

    pub fn evaluate(&self, word_letter_indices: &[(usize, usize)]) -> (u8, u8) {
        let mut word_multiplier = 1;
        let mut sum = 0;
        let mut gems = 0;
        for &(y, x) in word_letter_indices.iter() {
            let letter_data = self.data[y][x];
            sum += letter_data.pure_value * letter_data.letter_multiplier;
            word_multiplier = word_multiplier.max(letter_data.word_multiplier);
            gems += letter_data.has_gem as u8;
        }
        // Long word bonus before or after word_multiplier?
        sum += if word_letter_indices.len() > 5 { 10 } else { 0 };

        (sum * word_multiplier, gems)
    }

    pub fn evaluate_swapped(&self, word: &str, word_letter_indices: &[(usize, usize)]) -> (u8, u8) {
        let mut word_multiplier = 1;
        let mut sum = 0;
        let mut iter = word.chars();
        let mut gems = 0;
        for &(y, x) in word_letter_indices.iter() {
            let letter_data = self.data[y][x];
            let current_char = iter.next().unwrap();
            let pure_value = if current_char == self.characters[y][x] {
                letter_data.pure_value
            } else {
                evaluate(current_char)
            };
            sum += pure_value * letter_data.letter_multiplier;
            word_multiplier = word_multiplier.max(letter_data.word_multiplier);
            gems += letter_data.has_gem as u8;
        }
        // Long word bonus before or after word_multiplier?
        sum += if word_letter_indices.len() > 5 { 10 } else { 0 };

        (sum * word_multiplier, gems)
    }

    pub fn _trace(&self, word_path: &[(usize, usize)]) {
        for y in 0..self.characters.len() {
            for x in 0..self.characters[y].len() {
                if word_path.contains(&(y, x)) {
                    print!("\x1b[32m{}\x1b[0m ", self.characters[y][x]);
                } else {
                    print!("{} ", self.characters[y][x]);
                }
            }
            println!();
        }
        println!();
    }

    pub fn trace_swapped(&self, word: &str, word_path: &[(usize, usize)]) -> String {
        let mut trace = String::new();
        for y in 0..self.characters.len() {
            for x in 0..self.characters[y].len() {
                // node is part of the word
                if word_path.contains(&(y, x)) {
                    // let current_char = word_iter.next().unwrap();
                    let cc = word_path.iter().position(|&node| node == (y, x)).unwrap();
                    let current_char = word[cc..=cc].chars().next().unwrap();
                    let is_swapped = current_char != self.characters[y][x];
                    if is_swapped {
                        trace.push_str(&format!("\x1b[31m{}\x1b[0m ", current_char));
                    } else {
                        trace.push_str(&format!("\x1b[32m{}\x1b[0m ", self.characters[y][x]));
                    }
                } else {
                    trace.push_str(&format!("{} ", self.characters[y][x]));
                }
            }
            trace.push('\n');
        }
        trace
    }
    // fn log(&self) {
    //     for (i, ele) in self.characters.iter().enumerate() {
    //         for (j, c) in ele.iter().enumerate() {
    //             println!(
    //                 "{i}, {j}: {:?}",
    //                 (
    //                     c,
    //                     self.data[i][j],
    //                     self.get_neighbours((j, i))
    //                         .iter()
    //                         .map(|(x, y)| self.characters[*y][*x])
    //                         .collect::<Vec<_>>()
    //                 )
    //             );
    //         }
    //         // println!("{} {:?}", i, (ele, self.data[i]));
    //     }
    // }
}

pub fn _find_distance_betwixt(word_a: &str, word_b: &str) -> u8 {
    let (a_len, b_len) = (word_a.len(), word_b.len());
    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    for y in 0..a_len + 1 {
        for x in 0..b_len + 1 {
            // Base cases
            if y == 0 {
                matrix[y][x] = x as u8;
            } else if x == 0 {
                matrix[y][x] = y as u8;
            } else if word_a[y - 1..=y - 1] == word_b[x - 1..=x - 1] {
                // If both characters are equal
                matrix[y][x] = matrix[y - 1][x - 1];
            } else {
                matrix[y][x] = 1 + std::cmp::min(
                    matrix[y - 1][x],
                    std::cmp::min(matrix[y][x - 1], matrix[y - 1][x - 1]),
                );
            }
        }
    }

    matrix[a_len][b_len]
}

pub fn _find_distance_betwixt_optimised(word_a: &str, word_b: &str) -> u8 {
    let m = word_a.len();
    let n = word_b.len();
    let word_a = word_a.as_bytes();
    let word_b = word_b.as_bytes();

    let mut matrix: [Vec<u8>; 2] = [vec![0; n + 1], vec![0; n + 1]];

    for i in 0..=n {
        matrix[0][i] = i as u8;
    }

    for i in 1..=m {
        matrix[i % 2][0] = i as u8;
        for j in 1..=n {
            // if they're the same, then their cost is the previous cost (-1, -1)
            if word_a[i - 1] == word_b[j - 1] {
                matrix[i % 2][j] = matrix[(i - 1) % 2][j - 1];
            } else {
                // otherwise, the cost is the lowest cost so far + 1
                matrix[i % 2][j] = 1 + matrix[(i - 1) % 2][j]
                    .min(matrix[i % 2][j - 1])
                    .min(matrix[(i - 1) % 2][j - 1]);
            }
        }
    }

    matrix[m % 2][n]
}

#[inline(always)]
pub fn find_distance_betwixt_optimisedv2(word_a: &str, word_b: &str) -> u8 {
    let m = word_a.len();
    let n = word_b.len();
    let word_a = word_a.as_bytes();
    let word_b = word_b.as_bytes();

    let mut matrix = vec![0; n + 1];

    for (i, ele) in matrix.iter_mut().enumerate().take(n + 1) {
        *ele = i as u8;
    }

    // for i in 0..=n {
    //     matrix[i] = i as u8;
    // }

    for i in 1..=m {
        let mut prev = i as u8;
        for j in 1..=n {
            let current = if word_a[i - 1] == word_b[j - 1] {
                matrix[j - 1]
            } else {
                1 + matrix[j - 1].min(matrix[j]).min(prev)
            };
            matrix[j - 1] = prev;
            prev = current;
        }
        matrix[n] = prev;
    }

    matrix[n]
}

fn get_neighbours(shape_vec: &[[char; 5]; 5], index: (isize, isize)) -> Vec<(usize, usize)> {
    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, 0),  // N
        (-1, 1),  // NE
        (0, 1),   // E
        (1, 1),   // SE
        (1, 0),   // S
        (1, -1),  // SW
        (0, -1),  // W
        (-1, -1), // NW
    ];

    let (rows, cols) = (shape_vec.len() as isize, shape_vec[0].len() as isize);
    let mut neighbours: Vec<(usize, usize)> = Vec::with_capacity(8);
    for (dy, dx) in DIRECTIONS {
        let (y, x) = (index.0 + dy, index.1 + dx);
        if (y < 0 || y >= rows) || (x < 0 || x >= cols) {
            continue;
        }

        let new_index: (usize, usize) = (y as usize, x as usize);
        neighbours.push(new_index);
    }

    neighbours
}

// #[inline(always)]
const fn evaluate(letter: char) -> u8 {
    match letter {
        'a' | 'e' | 'i' | 'o' => 1,
        'n' | 'r' | 's' | 't' => 2,
        'd' | 'g' | 'l' => 3,
        'b' | 'h' | 'p' | 'm' | 'u' | 'y' => 4,
        'c' | 'f' | 'v' | 'w' => 5,
        'k' => 6,
        'j' | 'x' => 7,
        'q' | 'z' => 8,
        _ => 0,
    }
}
