pub fn encode(n: u64) -> String {
    if n == 0 {
    	return String::from("zero");
    }

    let word_chunks = vec!["", "thousand", "million", "billion", "trillion"];
    let num_chunks = chunkify(n);
    println!("{:?}", num_chunks);

    let mut word = String::from("");

    for (idx, chunk) in num_chunks.iter().enumerate() {
    	if *chunk > 0 {
	    	word = format!("{} {} {}", spell_hundreds(*chunk).unwrap(), 	word_chunks[idx], word);
    	}
    	println!("{:?}", word);
    }

    return word.trim().to_string();
}

fn spell_hundreds(n: u64) -> Option<String> {
	//basic cases
	let units = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
	let decs = vec!["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
	let n = n as usize;
	if n < 20 {
		return Some(units[n].to_string()); 
	} else if n < 100 {
		if n%10 != 0 {
			return Some(format!("{}-{}", decs[n/10-2], units[n%10]));
		} else {
			return Some(decs[n/10-2].to_string());
		}
	} else if n < 1000 {
		let mut word = String::from("");
		let n = n as u64;
		if n%100 != 0 {
			if let Some(decs) = spell_hundreds(n%100) {
				word = decs;
			}
		}
		let n = n as usize;
		word = format!("{} hundred {}", units[n/100], word);
		return Some(word.trim().to_string());
	} else {
		return None;
	}
}

fn chunkify(n: u64) -> Vec<u64> {
	if n == 0 {
		return vec![0];
	}
	let mut chunks = vec![];

	let mut rem = n;
	while rem > 0 {
		chunks.push(rem % 1000);
		rem = rem / 1000;
	}
	return chunks;
} 