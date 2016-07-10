extern crate regex;

use std::fs::File;
use std::collections::HashSet;
use std::path::Path;
use regex::Regex;
use std::error::Error;
use std::io::prelude::*;

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
}

fn words(haystack: &str) -> Vec<&str> {
	let re = Regex::new("[a-z]+").unwrap();
	let matches: Vec<&str> = re.split(haystack).collect();
	return matches;
}

// fn train(features: [String]) {

// }

// fn edits1(word: String) {

// }

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