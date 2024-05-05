// use rayon::prelude::*;
use std::collections::HashSet;
use std::io::{self};

mod dictionary;
mod shape;
use crate::shape::Graph;

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
    // const dictionary_string: &str = include_str!("../assets/words.txt");
    let dictionary_string =
        std::fs::read_to_string("assets/words.txt").expect("The words list should readable x(");

    let dict = dictionary::Dictionary::new(&dictionary_string);

    // let mut counts = dict
    //     .word_buckets
    //     .iter()
    //     .map(|(&k, v)| (k, v.len()))
    //     .collect::<Vec<(u8, usize)>>();
    // counts.sort();
    // println!("lengths: {:?}", counts);
    //

    let mut b = dict.word_buckets.keys().collect::<Vec<_>>();
    b.sort();
    println!("{b:?}");

    let word = "spaghetti";
    let swaps = 1;
    println!(
        "{}: {:?}",
        word.len(),
        (word.len() - swaps..word.len() + swaps)
    );
    for i in word.len() - swaps..=word.len() + swaps {
        println!("{i}");
    }

    let shape = std::fs::read_to_string("assets/shape_temp.txt")
        .expect("The 'shape.txt' file should be openable/readable x(");

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
            swapped_words.extend(swapped.iter());
        }
    }

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

    // TODO: make find_word_with_swaps only return 1 the first path and not multiple paths
    let swap_scores = swapped_words
        .iter()
        .map(|&s| {
            if let Some(path) = graph.find_word_with_swaps(s, 1) {
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

    for (s, ev, t, evt, path) in swap_scores.iter().flatten() {
        println!("{t}{s} {ev:?}->{evt:?} {path:?}\n");
    }

    // scores.retain(|pair| pair.1 .0 > 10);
    // for v in &scores[scores.len() - 5..] {
    //     println!("{:?}", (&v.0, v.1));
    //     graph.trace(v.2);
    // }

    // println!("{swapped_words:?}");
    // println!("{scores:?}");
    // print!("\x1b[2J\x1b[1;1H"); // clear console
    // let s = get_starting_matches(&"spa".to_string(), &dict.words);
    // println!("{s:?}");

    Ok(())
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
