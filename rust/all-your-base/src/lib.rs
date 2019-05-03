#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
	if from_base <= 1 { return Err(Error::InvalidInputBase); }
	else if to_base <= 1 { return Err(Error::InvalidOutputBase); }
    else {
    	let base10 = from_base_x_to_base_10(number, from_base);
    	println!("in base 10 {:?}", base10);
    	return base10.map(|n| from_base_10_to_base_x(n, to_base));
    }
}

fn from_base_x_to_base_10(number: &[u32], from_base: u32) -> Result<u32, Error> {
	number.iter().rev().enumerate().fold(Ok(0), |acc, (i, v)| if acc.is_ok() && v < &from_base { 
																acc.map( |n| n + v*from_base.pow(i as u32) ) 
															  } else if acc.is_ok() {
															  	Err(Error::InvalidDigit(*v))
															  } else {
															  	acc
															  })
}

fn from_base_10_to_base_x(number: u32, to_base: u32) -> Vec<u32> {
	let mut number = number;
	let mut base_x = vec![];

	while number > 0 {
		base_x.push(number % to_base);
		number /= to_base;
	}

	base_x.reverse();

	return base_x;
}