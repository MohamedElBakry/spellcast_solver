use rayon::prelude::*;
use std::cmp::min;
use std::collections::HashSet;
use std::io::{self};
use std::rc::Rc;

mod dictionary;
mod shape;
use crate::shape::Graph;

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

// TODO:
// 0. Fuzzy string search ✅
// 1. Sort by points (partially impemented) ✅
//  a. Evaluate function
// - if it's a valid prefix and the fuzzy matcher returns < 3 diff go
// - return approximate match if swapping a neighbour is possible (i.e, letter has valid neighbours
// that haven't already been used)
// 2. OCR
// 3. Support letter modifiers: DL, TL/ Double Word, Gems for sorting
// 4. Show top 5 words pathed on shape.txt
fn main() -> io::Result<()> {
    // Read file shape.txt
    // Traverse and Match to words.txt
    let dictionary_string =
        std::fs::read_to_string("assets/words.txt").expect("The words list should readable x(");
    // const dictionary_string: &str = include_str!("../assets/words.txt");
    // let dictionary_vec: Vec<&str> = dictionary_string.lines().collect();
    // const DS: &str = include_str!("../assets/collins.txt");
    // let dictionary_vec: Vec<&str> = DS.split('\n').collect();

    let dict = dictionary::Dictionary::new(&dictionary_string);

    // let mut counts = dict
    //     .word_buckets
    //     .iter()
    //     .map(|(&k, v)| (k, v.len()))
    //     .collect::<Vec<(u8, usize)>>();
    // counts.sort();
    // println!("lengths: {:?}", counts);

    let mut b = dict.word_buckets.keys().collect::<Vec<_>>();
    b.sort();
    println!("{b:?}");

    let w = "spaghetti";
    let swaps = 1;
    println!("{}: {:?}", w.len(), (w.len() - swaps..w.len() + swaps));
    for i in w.len() - swaps..w.len() + swaps + 1 {
        println!("{i}");
    }

    let shape = std::fs::read_to_string("assets/shape.txt")
        .expect("The 'shape.txt' file should be openable/readable x(");
    // println!("{}", shape::find_distance_betwixt("eeri", "please"));

    // let shape = std::fs::read_to_string("assets/shape.txt")?;
    let graph = Graph::new(&shape);
    for (i, row) in graph.characters.iter().enumerate() {
        println!("{i} {:?}", row);
    }

    let mut words = HashSet::new();
    let mut swapped_words = HashSet::new();
    for y in 0..5 {
        for x in 0..5 {
            let (valid, swapped) = graph.dfs_traverse((y, x), &dict);
            words.extend(valid);
            swapped_words.extend(swapped);
        }
    }

    // let mut words_set = words
    //     .iter()
    //     .map(|v| {
    //         v.iter()
    //             .map(|&(x, y)| graph.characters[x][y])
    //             .collect::<String>()
    //     })
    //     .collect::<HashSet<String>>();
    // // words_vec.sort_by_key(|word| word.len());
    // let swapped_strings = swapped_words.iter().map(|v| v.to_string()).collect();
    // let diff = words_set
    //     .symmetric_difference(&swapped_strings)
    //     .collect::<Vec<&String>>();
    // println!("{swapped_strings:?}");
    // println!("{:?}", graph.find_word_with_swaps("aloud", 2).len());
    // graph.find_word_with_swaps("aloud", 2);
    // println!("{:?}", (&diff, diff.len()));
    // println!("{words_set:?} - len {}", words_set.len());
    // println!(" - len: {}", swapped_words.len());
    // println!(
    //     "{} - {} = {} duplicates",
    //     (words_set.len() + swapped_words.len()),
    //     (diff.len()),
    //     (words_set.len() + swapped_words.len()) - diff.len()
    // );
    // get number of duplicates across both hashsets

    // Evaluate and sort words
    let mut scores = words
        .iter()
        .map(|indices| {
            (
                indices
                    .iter()
                    .map(|&(y, x)| graph.characters[y][x])
                    .collect::<String>(),
                graph.evaluate(indices),
                indices,
            )
        })
        .collect::<Vec<_>>();
    //
    scores.sort_by_key(|&(_, value, _)| value);
    scores.dedup_by_key(|(s, _, _)| s.clone());

    // let swap_scores = swapped_words
    //     .iter()
    //     .map(|&s| {
    //         let path = graph
    //             .find_word_with_swaps(s, 1)
    //             .first()
    //             .unwrap()
    //             .clone()
    //             .unwrap();
    //         let trace = graph.trace_swapped(s, &path);
    //
    //         (
    //             s,
    //             graph.evaluate(&path),
    //             trace,
    //             graph.evaluate_swapped(s, &path),
    //             path,
    //         )
    //     })
    //     .collect::<Vec<_>>();

    // for (s, ev, t, evt, path) in swap_scores {
    //     println!("{t}{s} {ev}->{evt} {path:?}\n");
    // }
    // scores.retain(|pair| pair.1 > 10);
    for v in &scores[scores.len() - 5..] {
        println!("{:?}", (&v.0, v.1));
        graph.trace(v.2);
    }
    // println!("{swapped_words:?}");
    // println!("{scores:?}");

    // Old
    // let result: Vec<(usize, usize)> = (0..5).flat_map(|x| (0..5).map(move |y| (x, y))).collect();
    // let r = result
    //     .iter()
    //     .map(|&(x, y)| traverse_dfs(&graph.characters, &dict.words, (x, y), 2))
    //     .collect::<Vec<Option<(Vec<Vec<(usize, usize)>>, Vec<Vec<(usize, usize)>>)>>>();

    // let mut words: HashSet<Vec<(usize, usize)>> = HashSet::new();
    // let mut swaps: HashSet<Vec<(usize, usize)>> = HashSet::new();
    //
    // for option in r.into_iter().flatten() {
    //     let (word_v, swap_v) = option;
    //
    //     words.extend(word_v);
    //     swaps.extend(swap_v);
    // }
    //
    // let mut ordered_v = words
    //     .iter()
    //     .map(|word_path| {
    //         word_path
    //             .iter()
    //             .map(|&(x, y)| graph.characters[x][y].to_string())
    //             .collect::<String>()
    //     })
    //     .collect::<Vec<String>>();
    // // .into_iter()
    // // .map(|word| (word.clone(), evaluate(&word)))
    // // .collect::<Vec<(String, u8)>>();
    //
    // // ordered_v.sort_by_key(|k| k.1);
    // // ordered_v.dedup();
    // println!("{:?} {}", ordered_v, ordered_v.len());
    // println!("\x1b[32m{:?}!\x1b[0m", ordered_v.last().unwrap());

    // dfs_true("", &shape_vec, (0, 0), &mut HashSet::new());

    // print!("\x1b[2J\x1b[1;1H"); // clear console

    Ok(())
}

/// Returns two vectors of vectors containing the indices of the letters in `shape_vec`
/// that form a valid word. The first vector is purely valid words and the second contains swaps.
/// In the case of the second vector, words with letters swaps, if a letter in the word doesn't match the
/// letter in `shape_vec`, then that's where the swap occured.
fn traverse_dfs(
    shape_vec: &[[char; 5]; 5], words_vec: &[&str], index: (usize, usize), max_swaps: u8,
) -> Option<(Vec<Vec<(usize, usize)>>, Vec<Vec<(usize, usize)>>)> {
    // let mut valid_words: Vec<String> = Vec::new();
    let mut visited = HashSet::with_capacity(9); // avg 8.68
    visited.insert(index);
    let (valid_words, words_with_letter_swaps) = dfs(
        shape_vec,
        words_vec,
        index,
        /* shape_vec[index.0][index.1].to_string(), */
        vec![index],
        &mut visited,
        max_swaps,
    );

    // print!("{:?} ", valid_words.len());
    if !valid_words.is_empty() {
        Some((valid_words, words_with_letter_swaps))
    } else {
        None
    }
}

// TODO: Separate dfs call for valid_swaps to trace out path.
fn dfs(
    shape_vec: &[[char; 5]; 5], words_vec: &[&str], index: (usize, usize),
    mut word_letter_indices: Vec<(usize, usize)>, visited: &mut HashSet<(usize, usize)>,
    max_swaps: u8,
) -> (Vec<Vec<(usize, usize)>>, Vec<Vec<(usize, usize)>>) {
    let mut valid_words: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut words_with_letter_swaps: Vec<Vec<(usize, usize)>> = Vec::new();

    let converted_word: String = word_letter_indices
        .iter()
        .map(|(x, y)| shape_vec[*x][*y].to_string())
        .collect::<Vec<String>>()
        .join("");
    // println!("{:?}->{:?}", word, converted_word);
    let neighbours = get_neighbours(shape_vec, (index.0 as isize, index.1 as isize))
        .into_iter()
        .filter(|n| !visited.contains(n))
        .collect::<Vec<(usize, usize)>>();
    for neighbour in &neighbours {
        // if visited.contains(neighbour) {
        //     // println!("\nalready visited {:?} on path of {:?} at {:?}", (neighbour, &shape_vec[neighbour.0][neighbour.1]), word, (index, &shape_vec[index.0][index.1]));
        //     continue; // we've included this letter in our word already so skip it.
        // }

        let neighbour_letter = &shape_vec[neighbour.0][neighbour.1];
        let prefix = format!("{converted_word}{neighbour_letter}");
        // println!("{prefix}");

        // println!("{:?}", (&prefix, potential_matches));
        // Consider whether words are being missed by only getting starting matches, and the
        // neighbour check len > n check.
        // let potential_matches = get_starting_matches(&prefix, words_vec);
        // let valid_swaps: Vec<String> = potential_matches
        //     .par_iter()
        //     .filter(|&w| find_distance_betwixt(w, &prefix) <= max_swaps)
        //     .map(|w| w.to_string())
        //     .collect();
        // println!("{:?}", (&prefix, valid_swaps.iter().map(|word| (word, evaluate(word))).collect::<Vec<_>>()));
        // println!("{:?}->{:?}={:?}", converted_word, neighbour_letter, prefix);
        if is_valid_prefix(&prefix, words_vec)
        /* || !valid_swaps.is_empty()  */
        {
            // if it is an actual word, add it and move forward further
            if words_vec.binary_search(&prefix.as_str()).is_ok() {
                word_letter_indices.push(*neighbour);
                valid_words.push(word_letter_indices.clone());
                word_letter_indices.pop();
                // println!("{:?} = {:?}?", converted_word, word);
            }

            visited.insert(*neighbour);
            word_letter_indices.push(*neighbour);
            let (valid_words_cache, words_with_swaps_cache) = &dfs(
                shape_vec,
                words_vec,
                *neighbour,
                word_letter_indices.clone(),
                visited,
                max_swaps,
            );
            valid_words.extend_from_slice(valid_words_cache);
            words_with_letter_swaps.extend_from_slice(words_with_swaps_cache);

            visited.remove(neighbour);
            word_letter_indices.pop();
        }
    }

    (valid_words, words_with_letter_swaps)
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

// fn dfs_true(
//     word: &str, shape_vec: &[Vec<String>], index: (usize, usize),
//     visited: &mut HashSet<(usize, usize)>,
// ) {
//     visited.insert(index);
//     for neighbour in get_neighbours(shape_vec, (index.0 as isize, index.1 as isize)) {
//         println!("{:?}->{:?}", shape_vec[index.0][index.1], (shape_vec[neighbour.0][neighbour.1]));
//         if visited.contains(&neighbour) {
//             continue;
//         }
//         dfs_true(word, shape_vec, neighbour, visited);
//     }
// }

// Levenshtein distance algorithm
fn find_distance_betwixt(word_a: &str, word_b: &str) -> u8 {
    let (ly, lx) = (word_a.len(), word_b.len());
    let mut matrix = vec![vec![0u8; lx + 1]; ly + 1];

    for y in 0..ly + 1 {
        for x in 0..lx + 1 {
            if y == 0 {
                matrix[y][x] = x as u8;
            } else if x == 0 {
                matrix[y][x] = y as u8;
            } else if word_a[y - 1..=y - 1] == word_b[x - 1..=x - 1] {
                matrix[y][x] = matrix[y - 1][x - 1];
            } else {
                matrix[y][x] = 1 + min(
                    min(matrix[y - 1][x], matrix[y][x - 1]),
                    matrix[y - 1][x - 1],
                )
            }
        }
    }

    matrix[ly][lx]
}

fn get_indices_starting_match(search_term: &String, words_vec: &[&str]) -> Vec<usize> {
    let mut matches: Vec<usize> = Vec::new();

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
        // matches.extend_from_slice(&words_vec[left..=right]);
        matches.extend(left..right);
    }

    matches
}

fn get_starting_matches<'a>(search_term: &String, words_vec: &'a [&'a str]) -> Vec<&'a str> {
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

fn evaluate(word: &str) -> u8 {
    let mut sum = word
        .chars()
        .map(|c| match c {
            'a' | 'e' | 'i' | 'o' => 1,
            'n' | 'r' | 's' | 't' => 2,
            'd' | 'g' | 'l' => 3,
            'b' | 'h' | 'p' | 'm' | 'u' | 'y' => 4,
            'c' | 'f' | 'v' | 'w' => 5,
            'k' => 6,
            'j' | 'x' => 7,
            'q' | 'z' => 8,
            _ => 0,
        })
        .sum();
    sum += if word.len() > 5 { 10 } else { 0 }; // Long word bonus for lengths of > 5

    sum
}
