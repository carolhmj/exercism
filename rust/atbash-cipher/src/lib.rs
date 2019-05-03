static ALPHABET_LAST_INDEX : usize = 25;
static ALPHABET : [char; 26] = ['a','b','c','d','e','f','g',
							    'h','i','j','k','l','m','n',
							    'o','p','q','r','s','t','u',
							    'v','w','x','y','z'];
							   

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.chars()
    	.filter(|c| c.is_ascii_alphanumeric())
    	.map(|c| c.to_ascii_lowercase())
    	.map(|c| match ALPHABET.iter().position(|&x| x == c) {
    				Some(pos) => ALPHABET[ALPHABET_LAST_INDEX - pos],
    				None => c,
    			})
    	.enumerate()
    	.fold(String::new(), |acc, (i, c)| {
    		let mut next = acc + &c.to_string();
    		if (i+1) % 5 == 0 {
    			next = next + " ";
    		}
    		next
    	})
    	.trim()
    	.to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars()
    	.filter(|c| c.is_ascii_alphanumeric())
    	.map(|c| match ALPHABET.iter().position(|&x| x == c) {
    				Some(pos) => ALPHABET[ALPHABET_LAST_INDEX - pos],
    				None => c,
    			})
    	.collect::<String>()
}
