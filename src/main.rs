use std::collections::{VecDeque, HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
// use std::time::{self};
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

fn main() -> io::Result<()> {
    // Read file shape.txt
    // Traverse and Match to words.txt

    let file = File::open("words.txt")
        .expect("The 'words.txt' file should be openable/readable to find valid words x()");
    let words_reader = io::BufReader::new(file);
    let words_vec: Vec<String> = words_reader.lines().map(|line| line.unwrap()).collect();

    let search_term: String = "zo".to_string();
    let matches = get_starting_matches(&search_term, &words_vec);
    println!("{:?}", &matches);

    // println!("{:?}", words_vec.binary_search(&"zoo".to_string()));
    let shape = File::open("shape.txt").expect("The 'shape' file should be openable/readable x(");
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

    for (i, ele) in shape_vec.iter().enumerate() {
        println!("{} {:?}", i, ele);
    }
    let mut s = Vec::from([1, 2, 3]);
    while let Some(a) = s.pop() {
        println!("{:?}", a);   
    }
    println!("done");
    // let mut full_words: Vec<Vec<String>> = Vec::new();
    // for x in 0..shape_vec.len() {
    //     for y in 0..shape_vec[x].len() {
    //         if let Some(words) = traverse_bfs(&shape_vec, &words_vec, (x, y)) {
    //             full_words.push(words);
    //         }
    //             // println!("{:?}", words);
    //     } 
    // }
    // println!("{:?}", full_words);
 
    let result = traverse_bfs(&shape_vec, &words_vec, (0,0));
    println!("{:?}", result);
    // println!("{:?}", get_starting_matches(&"ip".to_string(), &words_vec));
    // println!("{:?}", &get_starting_matches(&"ip".to_string(), &words_vec));
    // let words = traverse(&shape_vec, &words_vec, (0, 3), &mut "".to_string());
/*     const INDEX: (isize, isize) = (2, 2);
    let neighbours = get_neighbours(&shape_vec, INDEX);
    let s_neighbours = neighbours
        .iter()
        .map(|&ni| &shape_vec[ni.0][ni.1])
        .collect::<Vec<&String>>();

    println!(
        "{:?}'s neighbours = {:?}",
        (INDEX, &shape_vec[INDEX.0 as usize][INDEX.1 as usize]),
        &s_neighbours
    );
 */
    Ok(())
}


fn traverse_bfs(shape_vec: &[Vec<String>], words_vec: &[String], index: (usize, usize)) -> Option<Vec<String>> {
    let mut valid_words: Vec<String> = Vec::with_capacity(8);
    // let mut visited: HashSet<(usize, usize)> = HashSet::with_capacity(25);
    let mut visited: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::with_capacity(25);
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    queue.push_back(index);
    // visited.insert(index, )
    visited.insert(index, Vec::with_capacity(8));
    let mut count: u16 = 0;
    let mut word: String = String::new();
    // Store a Vector of strings containing the valid words during the BFS/DFS 
    // I -> P ✅ ["IP"]
    // I -> P -> X ❌ ["IP"]
    // I -> P -> E ✅ ["IPE"] (IPECAC)
    // P -> E . . . ✅ ["PE"] (PEEL)
    println!("sdfg");
    while let Some(node) = queue.pop_front() {
        // visited.insert(node);
        // println!("visited: {:?}", &shape_vec[node.0][node.1]);
        count += 1;
        println!("current: {:?}", (node, &word));
        for neighbour in &get_neighbours(shape_vec, (node.0 as isize, node.1 as isize)) {
            let visited_vals = visited.get(&node)?;
            if visited_vals.contains(neighbour) { println!("previsited {:?}->{:?}", node, neighbour); continue; }
            visited.entry(node).or_insert(Vec::from([*neighbour])).push(*neighbour);
            println!("{:?}", visited);
            let neighbour_letter = &shape_vec[neighbour.0][neighbour.1];
            // println!("{:?}->{:?}", &shape_vec[node.0][node.1], &shape_vec[neighbour.0][neighbour.1]);
            let mut new_combined = format!("{word}{neighbour_letter}");

            // println!("{word}->{combined}");
            if is_potential_word(&new_combined, words_vec) {
                let potential_matches = &get_starting_matches(&new_combined, words_vec);
                println!("{new_combined}->?{:?}", potential_matches.get(0..3).unwrap_or(potential_matches));
                word = new_combined.clone();
                // queue.push_front(*neighbour);
                println!("{:?}", queue);
            } else {
                if words_vec.binary_search(&word).is_err() {
                    // word.pop();
                    // new_combined.pop();
                    // word.clear();
                    // continue;
                }
                println!("{word} added and cleared");
                valid_words.push(word.clone());
                // word.clear();
                // word.pop();
                // visited.clear();
            }
        }
    }
    println!("asdf");
    println!("count = {count}, {:?}", valid_words);
    if !valid_words.is_empty() {
        Some(valid_words)
    } else {
        None
    }
}

fn is_potential_word(search_term: &String, words_vec: &[String]) -> bool {
    words_vec.binary_search_by(|word| {
        if word.starts_with(search_term) {
            std::cmp::Ordering::Equal
        } else {
            word.cmp(search_term)
        }
    }).is_ok()
}




// #[allow(dead_code)]
fn traverse(
    shape_vec: &[Vec<String>], words_vec: &[String], index: (isize, isize), old_string: &mut String,
) -> Option<Vec<String>> {
    // Base case: valid word sequence?
    // Get children
    // for child in children: traverse/visit
    // let visited: Vec<(usize, usize)> = Vec::with_capacity(8);
    // let matches = get_starting_matches(combined, words_vec);
    // if matches.is_empty() {
    // combined
    // }

    let mut current_string_variations: Vec<String> = Vec::with_capacity(3);
    let mut current_string = shape_vec[index.0 as usize][index.1 as usize].to_string();
    // old_string.push_str(&current_string);
    current_string = format!("{old_string}{current_string}"); 
    // current_string.push_str(old_string);
    println!("{current_string} = current");

    if let Ok(_) = words_vec.binary_search_by(|word| {
        if word.starts_with(&current_string) {
            println!("found: {word}");
            std::cmp::Ordering::Equal
        } else {
            word.cmp(&current_string)
        }
    }) {
        current_string_variations.push(current_string.clone());
        let neighbours = get_neighbours(shape_vec, index);
        for neighbour in neighbours {
            println!("{:?}", neighbour);
            if let Some(variations) = traverse(
                shape_vec,
                words_vec,
                (neighbour.0 as isize, neighbour.1 as isize),
                &mut current_string,
            ) {
                // current_string_variations.push(variation);
                current_string_variations.extend_from_slice(&variations);
            }
        }
    }

    Some(current_string_variations)
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
    for direction in DIRECTIONS {
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

fn get_starting_matches(search_term: &String, words_vec: &[String]) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();

    // Get the index of the first word that starts with the search_term in the vector
    if let Ok(index) = words_vec.binary_search_by(|word| {
        if word.starts_with(search_term) {
            std::cmp::Ordering::Equal
        } else {
            word.cmp(search_term)
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
// #[cfg(test)]
// mod tests {
//     use std::fs;
//
//     use super::*;
//
//     #[test]
//     fn read() -> io::Result<()> {
//         // let file = File::open("words.txt")?;
//         // let reader = io::BufReader::new(file);
//         // for line in reader.lines() {
//         //     println!("{} {}", line?, random());
//         // }
//         //
//         let content = fs::read_to_string("words.txt");
//         println!("{:?}", content);
//         Ok(())
//     }
// }
