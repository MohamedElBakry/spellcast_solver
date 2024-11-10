// use rayon::prelude::*;
use std::collections::HashSet;
use std::io::{self};

mod dictionary;
mod shape;
use crate::shape::Graph;

fn main() -> io::Result<()> {
    // Read file shape.txt
    // Traverse and Match to words.txt
    // const dictionary_string: &str = include_str!("../assets/words.txt");
    let dictionary_string =
        std::fs::read_to_string("assets/words.txt").expect("The words list should readable x(");

    let dict = dictionary::Dictionary::new(&dictionary_string);

    let shape = std::fs::read_to_string("assets/shape_temp.txt")
        .expect("The 'shape.txt' file should be openable/readable x(");

    let graph = Graph::new(&shape);
    for (i, row) in graph.characters.iter().enumerate() {
        println!("{i} {:?}", row);
    }

    const SWAPS: u8 = 1;
    let mut words = HashSet::new();
    let mut swapped_words = HashSet::new();
    for y in 0..5 {
        for x in 0..5 {
            let (valid, swapped) = graph.dfs_traverse((y, x), SWAPS, &dict);
            words.extend(valid);
            swapped_words.extend(swapped.iter());
        }
    }

    println!("{:?}", (swapped_words.len(), words.len()));
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

    scores.sort_by_key(|&(_, value, _)| value);
    scores.dedup_by_key(|(s, _, _)| s.clone());

    // TODO: get top 5 words of swapped and not swapped
    // TODO: make find_word_with_swaps only return 1 the first path and not multiple paths
    let mut swap_scores = swapped_words
        .iter()
        .filter_map(|&s| {
            if let Some(path) = graph.find_word_with_swaps(s, SWAPS as i8) {
                let trace = graph.trace_swapped(s, &path);
                Some((
                    s,
                    graph.evaluate(&path),
                    trace,
                    graph.evaluate_swapped(s, &path),
                    path,
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    swap_scores.sort_unstable_by_key(|(_, _, _, value, _)| *value);
    for (word, ev, highlighted_grid, value, path) in swap_scores {
        // println!("{highlighted_grid}{word} {ev:?}->{value:?} {path:?}\n");
        println!("{highlighted_grid}{word} {value:?}\n");
    }

    Ok(())
}

fn _get_indices_starting_match(search_term: &String, words_vec: &[&str]) -> Vec<usize> {
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

fn _get_starting_matches<'a>(search_term: &String, words_vec: &'a [&'a str]) -> Vec<&'a str> {
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
