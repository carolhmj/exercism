pub fn check(candidate: &str) -> bool {
	let candidate = candidate.to_lowercase();
    let chars = candidate.char_indices();
    for (i, c) in chars {
    	if c == '-' || c == ' ' {
    		continue;
    	}
    	if let Some(li) = candidate.rfind(c) {
    		if i != li {
    			return false;
    		}
    	}
    }
    return true;
}