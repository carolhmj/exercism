pub fn reply(message: &str) -> &str {
	println!("message: {}", message);
    if is_question(message) {
    	if is_yelling(message) {
    		return "Calm down, I know what I'm doing!"
    	} else {
    		return "Sure."
    	}
    } else {
    	if is_yelling(message) {
    		return "Whoa, chill out!"
    	} else if is_silence(message) {
    		return "Fine. Be that way!"
    	} else {
    		return "Whatever."
    	}
    }
}

//a message is silence if all its characters are whitespace 
fn is_silence(message : &str) -> bool {
	for letter in message.chars() {
		if !letter.is_whitespace() {
			return false
		}
	}
	return true
}

//a message is a yell if it has alpabetical characters and they are all
//uppercase
fn is_yelling(message: &str) -> bool {
	let mut n_alpha = 0;
	let mut n_uppercase = 0;
	for letter in message.chars() {
		if letter.is_alphabetic() {
			n_alpha = n_alpha + 1;
			if letter.is_uppercase() {
				n_uppercase = n_uppercase + 1;
			}
		}
	}
	n_alpha != 0 && n_alpha == n_uppercase
}

//a message is a question if '?' is its last character, or all characters
//after '?' are whitespace
fn is_question(message: &str) -> bool {
	match message.find("?") {
		Some(index) => is_silence(&message[index+1..]),
		None => false,
	}
}