/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
    	return None;
    }
	return Some(s1.chars().enumerate().fold(0, |acc, (i, c)| if (s2.as_bytes())[i] as char != c {acc+1} else {acc}));
}
