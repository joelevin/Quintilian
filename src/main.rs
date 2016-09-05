extern crate regex;

use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::iter::Iterator;
use std::env;

fn main() {

    if let Some(arg1) = env::args().nth(1) {
		// read file
		let path = Path::new("big.txt");
		let display = path.display();
		let mut file = match File::open(&path) {
			Err(why) => panic!("couldn't open {}: {}", display, why.description()),
			Ok(file) => file,
		};

		// parse into string
		let mut buffer = String::new();
		file.read_to_string(&mut buffer).unwrap();
	    let nwords = words(&buffer);
	    let known_words = train(nwords);

	    let correct_word = correct(&arg1, known_words);
	    println!("{:?}", correct_word);
    }
    // deletes_test(split_test("hello"));
    // transposes_test(split_test("hello"));
    // replace_test(split_test("hello"));
    // inserts_test(split_test("hello"));
}

fn words(buffer: &str) -> Vec<&str> {
	return buffer.split_whitespace().collect();
}

fn train(features: Vec<&str>) -> HashMap<&str, i32> {
	let mut model = HashMap::new();

	for feature in features {
		match model.entry(feature) {
			Occupied(mut entry) => { (*entry.get_mut()) += 1; },
			Vacant(entry) => { entry.insert(1); },
		}
	}

	return model;
}

fn splits(word: &str) -> Vec<(&str, &str)> {
	let splits: Vec<(&str, &str)> = word.char_indices().map(|index| {
		return word.split_at(index.0);
	}).collect();

	return splits;
}

fn deletes(splits: &Vec<(&str, &str)>) -> Vec<String> {
	return splits.clone().iter().map(|split| {
		let concatenated_string: String = split.0.to_owned();
		let mut second_split: String = split.1.to_owned();
		second_split.remove(0);
		return concatenated_string + &second_split;
	}).collect();
}

fn edits1(word: &str) -> HashSet<String> {
	let splits = splits(word);
	let deletes: Vec<String> = deletes(&splits);

	let transposes: Vec<String> = splits.clone().iter().map(|split| {
		let left_string: String = split.0.to_owned();
		let right_string: String = split.1.to_owned();
		if let Some(first) = right_string.chars().nth(0) {
			let first_string = first.to_string();
			let mut first_drop = right_string.clone();
			first_drop.remove(0);
			if let Some(second) = first_drop.chars().nth(0) {
				let second_string = second.to_string();
				let mut second_drop = first_drop.clone();
				second_drop.remove(0);
				return left_string + &second_string + &first_string + &second_drop;
			}
		}
		return "".to_string()
	}).filter(|string| !string.is_empty()).collect();

	// replaces
	let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut replaces: Vec<String> = Vec::new();
    for split in splits.clone() {
    	for letter in alphabet.chars() {
    		let mut first_drop = split.1.to_owned().clone();
    		first_drop.remove(0);
	    	replaces.push(split.0.to_owned() + &letter.to_string() + &first_drop);
	    }
    }

	// inserts
	let mut inserts: Vec<String> = Vec::new();
    for split in splits.clone() {
    	for letter in alphabet.chars() {
	    	inserts.push(split.0.to_owned() + &letter.to_string() + &split.1.to_owned());
	    }
    }

    let mut edits = HashSet::new();
    for delete in deletes {
    	edits.insert(delete);
    }
    for transpose in transposes {
    	edits.insert(transpose);
    }
    for replace in replaces {
    	edits.insert(replace);
    }
    for insert in inserts {
    	edits.insert(insert);
    }

    return edits;
}

fn known_edits_2(word: &str, nwords: HashMap<&str, i32>) -> Option<HashSet<String>> {
	let known_edits: HashSet<String> = HashSet::new();
	let mut union: HashSet<String> = HashSet::new();
	let edits_1 = edits1(word);
	for _ in edits_1 {
		if let Some(k) = known(edits1(word), nwords.clone()) {
			union = known_edits.union(&k).cloned().collect();
		}
	}

	if union.is_empty() {
		return None;
	}
	else {
	    return Some(union);
	}
}

fn known(words: HashSet<String>, nwords: HashMap<&str, i32>) -> Option<HashSet<String>> {
	let mut set = HashSet::new();
	let known_words: Vec<String> = words.into_iter().filter(|word| nwords.contains_key::<str>(&word)).collect();
	for word in known_words {
		set.insert(word);
	}

	if set.is_empty() {
		return None
	}
	else {
	    return Some(set);
	}
}

fn correct(word: &str, nwords: HashMap<&str, i32>) -> String {
	let mut word_set: HashSet<String> = HashSet::new();
	word_set.insert(word.to_string());
	let empty_set: HashSet<String> = HashSet::new();

	let first_known = known(word_set.clone(), nwords.clone());
	let first_known_edits = known(edits1(word), nwords.clone());
	let second_known_edits = known(known_edits_2(word, nwords.clone()).unwrap_or(empty_set.clone()), nwords.clone());

	let candidates = first_known.unwrap_or(first_known_edits.unwrap_or(second_known_edits.unwrap_or(empty_set)));
	let candidate = candidates.iter().max_by_key(|key| {
		let key_slice: &str = &key;
		let default_count: i32 = 1;
		let value = nwords.get(key_slice);
		// println!("{:?} : {:?}", key_slice, value);
		match value {
		    Some(value) => return value.to_owned(),
		    None => return default_count.to_owned(),
		};
	});
	// println!("candidates: {:?}",  candidates);
	// println!("candidate: {:?}", candidate);

	match candidate {
	    Some(candidate) => return candidate.to_string(),
	    None => return String::from(word),
	}
}

#[test]
fn split_test() {
	let word: &str = "hello";
	let splits = splits(word);
	let expected_splits: Vec<(&str, &str)> = vec![("", "hello"), ("h", "ello"), ("he", "llo"), ("hel", "lo"), ("hell", "o")];
	assert_eq!(&splits[..], &expected_splits[..]);
}

#[test]
fn deletes_test() {
	let word: &str = "hello";
	let splits = splits(word);
	let deletes = deletes(&splits);
	let expected_deletes = vec!["ello", "hllo", "helo", "helo", "hell"]; 
	assert_eq!(&deletes[..], &expected_deletes[..]);
}

// #[test]
// fn transposes_test(splits: Vec<(&str, &str)>) {
// 	let transposes: Vec<String> = splits.iter().map(|split| {
// 		let left_string: String = split.0.to_owned();
// 		let right_string: String = split.1.to_owned();
// 		if let Some(first) = right_string.chars().nth(0) {
// 			let first_string = first.to_string();
// 			let mut first_drop = right_string.clone();
// 			first_drop.remove(0);
// 			if let Some(second) = first_drop.chars().nth(0) {
// 				let second_string = second.to_string();
// 				let mut second_drop = first_drop.clone();
// 				second_drop.remove(0);
// 				return left_string + &second_string + &first_string + &second_drop;
// 			}
// 		}

// 		return "".to_string()
	
// 	}).filter(|string| !string.is_empty()).collect();

// 	println!("{:?}", transposes);
// }

// #[test]
// fn replace_test(splits: Vec<(&str, &str)>) {
//     let alphabet = "abcdefghijklmnopqrstuvwxyz";
//     let mut replaces: Vec<String> = Vec::new();
//     for split in splits {
//     	for letter in alphabet.chars() {
//     		let mut first_drop = split.1.to_owned().clone();
//     		first_drop.remove(0);
// 	    	replaces.push(split.0.to_owned() + &letter.to_string() + &first_drop);
// 	    }
//     }

//     println!("{:?}", replaces);
// }

// #[test]
// fn inserts_test(splits: Vec<(&str, &str)>) {
// 	let alphabet = "abcdefghijklmnopqrstuvwxyz";
//     let mut inserts: Vec<String> = Vec::new();
//     for split in splits {
//     	for letter in alphabet.chars() {
// 	    	inserts.push(split.0.to_owned() + &letter.to_string() + &split.1.to_owned());
// 	    }
//     }

//     println!("{:?}", inserts);
// }