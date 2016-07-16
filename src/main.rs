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

    deletesTest(splitTest("hello"));
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
		let mut concatenatedString: String = split.0.to_owned();
		let mut secondSplit: String = split.1.to_owned();
		secondSplit.remove(0);
		return concatenatedString + &secondSplit;
	}).collect();
}


fn splitTest(word: &str) -> Vec<(&str, &str)> {
	let splits: Vec<(&str, &str)> = word.char_indices().map(|(index, character)| {
		return word.split_at(index);
	}).collect();

	println!("{:?}", splits);

	return splits;
}

fn deletesTest(splits: Vec<(&str, &str)>) {
	let deletes: Vec<String> = splits.iter().map(|split| {
		let mut concatenatedString: String = split.0.to_owned();
		let mut secondSplit: String = split.1.to_owned();
		secondSplit.remove(0);
		return concatenatedString + &secondSplit;
	}).collect();

	println!("{:?}", deletes);
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