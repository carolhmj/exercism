extern crate regex;

use regex::Regex;

pub fn translate(input: &str) -> String {
	input.split_whitespace()
		.map(|word| format!("{} ", translate_word(word)))
		.collect::<String>()
		.trim().to_string()
}

fn translate_word(input: &str) -> String {
    //Follows Rule 1
    let r1 = Regex::new(r"^([aeiou]|yt|xr)\w*$").unwrap();
    if r1.find(input).is_some() {
    	let mut input = input.to_string();
    	input.push_str("ay");
    	return input;
    } 
    //For all the next rules, we need to be able to know the first consonant
    //or consonant cluster 
    let mut chars = input.chars().peekable();
    let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 
    'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'];
    let mut cluster = String::new();

    while let Some(ch) = chars.next() {
    	cluster.push(ch);			
    	if let Some(next) = chars.peek() {
    		let group = format!("{}{}", ch, *next);
    		if group != "qu" && !consonants.contains(&next) {
    			break;
    		}
    	}
    }

    return format!("{}{}ay", chars.collect::<String>(), cluster);
}
