/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
	let code = code.replace(" ", "");
    if code.len() <= 1 || code.chars().any(|c| !c.is_numeric()) {
    	return false;
    }
    return code.chars().rev().enumerate().map(|(i, c)| {
    let n : u32 = c.to_digit(10).unwrap();
    if i % 2  == 1 {
    		if n*2 > 9 {
    			n*2-9
    		} else {
   			n*2
   		}
    } else { 
    	n 
    }}).sum::<u32>() % 10 == 0
}
