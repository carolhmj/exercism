pub fn abbreviate(phrase: &str) -> String {
	//Remove special characters from phrase
	let phrase = phrase.chars()
		.map(|c| if !c.is_ascii_punctuation() { 
			c.to_string() 
		} else {" ".to_string()})
		.collect::<String>();
	println!("{:?}", phrase);	

	//Check if any word is an all-caps or all-lowercase. If there is one,
	//make it mixed case to make our life easier
	let phrase = phrase.split(" ")
				.map(|w| make_mixed(w))
				.collect::<Vec<String>>()
				.join(" ");	

	println!("{:?}", phrase);

	phrase.chars()
		.filter(|c| c.is_ascii_uppercase())
		.collect::<String>()			 	
}


fn make_mixed(word: &str) -> String {
	if word.chars().all(|c| c.is_ascii_uppercase()) {
		word.chars()
		.enumerate()
		.map(|(i, c)| if i > 0 { 
			c.to_lowercase().to_string() 
		} else { c.to_string() })
		.collect::<String>()
	} else if word.chars().all(|c| c.is_ascii_lowercase()) {
		word.chars()
		.enumerate()
		.map(|(i, c)| if i == 0 { 
			c.to_uppercase().to_string() 
		} else { c.to_string() })
		.collect::<String>()
	} else {
		word.to_string()
	}
}