pub fn encode(source: &str) -> String {
    let mut current_char_pos = 0;
    let mut destination = String::new();

    while current_char_pos < source.len() {
    	let next_char_pos = find_next_char(source, current_char_pos);
    	if current_char_pos < source.len() {
    		if next_char_pos-current_char_pos > 1 {
    			destination += &format!("{}{}", next_char_pos-current_char_pos, char_at(source, current_char_pos))
    		} else {
    			destination += char_at(source, current_char_pos);
    		}
    	}
    	current_char_pos = next_char_pos;
    }
    return destination;
}

//Given a character at pos "at", return the pos of the first character that is not equal to it
fn find_next_char(source: &str, at: usize) -> usize {
	for i in at+1..source.len() {
		if char_at(source, i) != char_at(source, i-1) {
			return i;
		}
	}
	return source.len();			
}

//Gives char at position 
fn char_at(source: &str, at: usize) -> &str {
	&source[at..at+1]
}

pub fn decode(source: &str) -> String {
    let chars = source.chars();
    let mut qty_string = String::new();
    let mut decoded_string = String::new();

    for c in chars {
    	if c.is_numeric() {
    		qty_string.push(c);
    	} else {
    		let qty : usize = qty_string.parse().unwrap_or(1);
    		decoded_string += c.to_string().repeat(qty).as_str();
    		qty_string.clear();
    	}
    }

    return decoded_string;
}
