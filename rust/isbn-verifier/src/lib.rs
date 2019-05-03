/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
	//First, let's remove the dashes
	let mut isbn = isbn.to_string();
	isbn.retain(|c| c != '-');

	//Check if the string has a valid ISBN 10 length
	if isbn.len() != 10 {
		return false;
	}

	//Let's turn this into numbers, taking care of the special last verification digit
    let n : Vec<Result<u32, std::num::ParseIntError>> = isbn.chars().enumerate()
    .map(|(i, x)| if x == 'X' && i == isbn.len()-1 {Ok(10)} else {x.to_string().parse()})
    .collect();

    //If there was an error in parsing, we know it's invalid
    if n.iter().any(|x| x.is_err()) {
    	return false;
    }
    //Check ISBN formula
    return (n.iter().flatten().fold((0, 10), |(acc, idx), &x| (acc + x*idx, idx-1)).0 % 11) == 0;
}
