use std::cmp::min;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufRead};

// #[allow(dead_code)]
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

// TODO:
// 0. Fuzzy string search
// 1. Sort by points
// 2. Combine multiple word files into one
// - if it's a valid prefix and the fuzzy matcher returns < 3 diff go
// - return approximate match if swapping a neighbour is possible (i.e, letter has valid neighbours
// that haven't already been used)
// 3. OCR
// 4. Support letter modifiers: DL, TL/ Double Word, Gems for sorting
fn main() -> io::Result<()> {
    // Read file shape.txt
    // Traverse and Match to words.txt
    let dictionary_string = fs::read_to_string("assets/words.txt").expect("The words list should readable x(");
    let dictionary_vec: Vec<&str> = dictionary_string.split('\n').collect();

    // let file = File::open("assets/collins.txt").expect("The 'words.txt' file should be openable/readable to find valid words x(");
    // let words_reader = io::BufReader::new(file);
    // let dictionary_vec: Vec<String> = words_reader.lines().map(|line| line.unwrap()).collect();

    let shape = File::open("assets/shape.txt").expect("The 'shape' file should be openable/readable x(");
    let reader = io::BufReader::new(shape);
    let shape_vec: Vec<Vec<String>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .trim()
                .split(' ')
                .map(|word| word.to_string())
                .collect()
        })
        .collect();

    // let shape_string = fs::read_to_string("assets/shape.txt").expect("The 'shape' file should be openable/readable x(");
    // println!("{shape_string}");
    // let shape_vec: Vec<Vec<&str>> = shape_string.split(&['\n']).map(|line| line.trim().split(' ').collect::<Vec<&str>>()).filter(|sa| !sa.contains(&"")).collect();

    for (i, ele) in shape_vec.iter().enumerate() {
        println!("{} {:?}", i, ele);
    }

    let mut words: HashSet<String> = HashSet::with_capacity(170);
    for x in 0..5 {
        for y in 0..5 {
            if let Some(word_v) = traverse_dfs(&shape_vec, &dictionary_vec, (x, y)) {
                words.extend(word_v);
            }
        }
    }

    // println!("{:?}", words);
    let mut ordered = Vec::from_iter(&words);

    ordered.retain(|w| w.len() > 4);
    ordered.sort_by_key(|k| k.len());
    println!(
        "words: {:?}",
        (/*&set,*/ words.len(), words.capacity(), ordered.len(), ordered)
    );

    Ok(())
}

fn traverse_dfs(
    shape_vec: &[Vec<String>], words_vec: &[&str], index: (usize, usize),
) -> Option<Vec<String>> {
    // let mut valid_words: Vec<String> = Vec::new();
    let visited = &mut HashSet::with_capacity(9); // avg 8.68
    visited.insert(index);
    let valid_words = dfs(
        shape_vec,
        words_vec,
        index,
        shape_vec[index.0][index.1].to_string(),
        visited,
    );

    // print!("{:?} ", valid_words.len());
    if !valid_words.is_empty() {
        Some(valid_words)
    } else {
        None
    }
}

fn dfs(
    shape_vec: &[Vec<String>], words_vec: &[&str], index: (usize, usize), mut word: String,
    visited: &mut HashSet<(usize, usize)>,
) -> Vec<String> {
    let mut valid_words: Vec<String> = Vec::with_capacity(3);
    let neighbours = get_neighbours(shape_vec, (index.0 as isize, index.1 as isize)); 
    for neighbour in &neighbours {
        if visited.contains(neighbour) {
            // println!("\nalready visited {:?} on path of {:?} at {:?}", (neighbour, &shape_vec[neighbour.0][neighbour.1]), word, (index, &shape_vec[index.0][index.1]));
            continue; // we've included this letter in our word already so skip it.
        }
        let neighbour_letter = &shape_vec[neighbour.0][neighbour.1][0..1];
        let prefix = format!("{word}{neighbour_letter}");
        let potential_matches = get_starting_matches(&prefix, words_vec);
        // Make more efficient by storing the result of find_distance on potential_matches
        // Consider whether words are being missed by only getting starting matches, and the
        // neighbour check len > n check. 
        let is_valid_with_swaps = potential_matches.iter().any(|&w| find_distance(w, &prefix) < 2);
        // print!("\n{:?}->{:?}={:?}", word, neighbour_letter, prefix);
        if is_valid_prefix(&prefix, words_vec) || is_valid_with_swaps {
            // print!("✅");
            word = prefix.clone();
            // if it is an actual word, add it and move forward further
            if words_vec.binary_search(&word.as_str()).is_ok() {
            // if is_valid_with_swaps && neighbours.len() > 1 {
                valid_words.push(word.clone());
            }

            if is_valid_with_swaps && neighbours.len() > 1 {
                valid_words.extend(potential_matches.iter().filter(|&w| find_distance(w, &prefix) < 2).map(|w| w.to_string()).collect::<Vec<String>>());
            }

            visited.insert(*neighbour);
            valid_words.extend_from_slice(&dfs(shape_vec, words_vec, *neighbour, word.clone(), visited));
            visited.remove(neighbour);
            word.pop();
        }
    }

    valid_words
}

fn is_valid_prefix(search_term: &String, words_vec: &[&str]) -> bool {
    words_vec
        .binary_search_by(|word| {
            if word.starts_with(search_term) {
                std::cmp::Ordering::Equal
            } else {
                word.cmp(&search_term.as_str())
            }
        })
        .is_ok()
}

fn get_neighbours(shape_vec: &[Vec<String>], index: (isize, isize)) -> Vec<(usize, usize)> {
    // get possible_neighbours
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

    // Get valid words, then only go forward if the neighbouring letter forms a valid combination.

    // let mut neighbours: [(usize, usize); 8] = [(index.0 as usize, index.1 as usize); 8];
    let mut neighbours: Vec<(usize, usize)> = Vec::with_capacity(8);
    // let mut neighbours: [&str; 8] = [""; 8];
    // let mut neighbours: Vec<String> = Vec::with_capacity(4);
    // Vec::with_capacity(capacity)
    for  direction in DIRECTIONS {
        let dir = direction_to_relative_index(&direction);
        let new_index = (index.0 + dir.0, index.1 + dir.1);
        // println!("{:?}", (new_index, shape_vec.len()));
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

// levenshtein distance algorithm
fn find_distance(word_a: &str, word_b: &str) -> u8 {
    let (ly, lx) = (word_a.len(), word_b.len());
    let mut matrix = vec![vec![0u8; lx + 1]; ly + 1];
    // print!("{:?}", matrix);

    for y in 0..ly + 1 {
        for x in 0..lx + 1 {
            if y == 0 {
                matrix[y][x] = x as u8;
            } else if x == 0 {
                matrix[y][x] = y as u8;
            } else if word_a[y - 1..=y - 1] == word_b[x - 1..=x - 1] {
                matrix[y][x] = matrix[y - 1][x - 1];
            } else {
                matrix[y][x] = 1 + min(min(matrix[y - 1][x], matrix[y][x - 1]), matrix[y - 1][x - 1])
            }
        }
    }

    // println!("{:?}", matrix);
    matrix[ly][lx]
}

fn get_starting_matches<'a>(search_term: &'a String, words_vec: &'a [&'a str]) -> Vec<&'a str> {
    let mut matches: Vec<&str> = Vec::new();

    // Get the index of the first word that starts with the search_term in the vector
    if let Ok(index) = words_vec.binary_search_by(|word| {
        if word.starts_with(search_term) {
            std::cmp::Ordering::Equal
        } else {
            word.cmp(&search_term.as_str())
        }
    }) {
        // Then, check if there were any words before or after it that also start with the search_term
        let mut left = index;
        let mut right = index;

        while left > 0 && words_vec[left - 1].starts_with(search_term) {
            left -= 1;
        }

        while right < words_vec.len() - 1 && words_vec[right + 1].starts_with(search_term) {
            right += 1;
        }

        // Index into the words_vec and the matches from the ranged slice.
        matches.extend_from_slice(&words_vec[left..=right]);
    }

    matches
}
