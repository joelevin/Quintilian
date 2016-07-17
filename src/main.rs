extern crate regex;

use std::fs::File;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::path::Path;
use regex::Regex;
use std::error::Error;
use std::io::prelude::*;
use std::iter::Iterator;

fn main() {

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
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    // deletes_test(split_test("hello"));
    // transposes_test(split_test("hello"));
    // replace_test(split_test("hello"));
    inserts_test(split_test("hello"));
}

fn words(haystack: &str) -> Vec<&str> {
	let re = Regex::new("[a-z]+").unwrap();
	let matches: Vec<&str> = re.split(haystack).collect();
	return matches;
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

fn edits1(word: &str) {
	let splits: Vec<(&str, &str)> = word.char_indices().map(|index| {
		return word.split_at(index.0);
	}).collect();

	let deletes: Vec<String> = splits.iter().map(|split| {
		let concatenated_string: String = split.0.to_owned();
		let mut second_split: String = split.1.to_owned();
		second_split.remove(0);
		return concatenated_string + &second_split;
	}).collect();

	let transposes: Vec<String> = splits.iter().map(|split| {
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
}


fn split_test(word: &str) -> Vec<(&str, &str)> {
	let splits: Vec<(&str, &str)> = word.char_indices().map(|(index, character)| {
		return word.split_at(index);
	}).collect();

	println!("{:?}", splits);

	return splits;
}

fn deletes_test(splits: Vec<(&str, &str)>) {
	let deletes: Vec<String> = splits.iter().map(|split| {
		let concatenated_string: String = split.0.to_owned();
		let mut second_split: String = split.1.to_owned();
		second_split.remove(0);
		return concatenated_string + &second_split;
	}).collect();

	println!("{:?}", deletes);
}

fn transposes_test(splits: Vec<(&str, &str)>) {
	let transposes: Vec<String> = splits.iter().map(|split| {
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

	println!("{:?}", transposes);
}

fn replace_test(splits: Vec<(&str, &str)>) {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut replaces: Vec<String> = Vec::new();
    for split in splits {
    	for letter in alphabet.chars() {
    		let mut first_drop = split.1.to_owned().clone();
    		first_drop.remove(0);
	    	replaces.push(split.0.to_owned() + &letter.to_string() + &first_drop);
	    }
    }

    println!("{:?}", replaces);
}

fn inserts_test(splits: Vec<(&str, &str)>) {
	let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut inserts: Vec<String> = Vec::new();
    for split in splits {
    	for letter in alphabet.chars() {
	    	inserts.push(split.0.to_owned() + &letter.to_string() + &split.1.to_owned());
	    }
    }

    println!("{:?}", inserts);
}
// fn known_edits2(word: String) {

// }

// fn known(words: [String], nwords: HashSet<String>) -> Option<[String]> {
// 	// for word in words {
// 	// 	if (nwords.contains())
// 	// }
// 	// None
// }

// fn correct(word: String) -> String {
	 
// }