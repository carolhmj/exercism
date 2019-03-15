#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|n| single_value_to_bytes(*n)).collect()
}

pub fn single_value_to_bytes(value: u32) -> Vec<u8> {
	let mut value = value;
	let mask : u32 = 0x7F;
	let mut result = vec![];
	
	for _i in 0..5 {
		let byte = (value & mask) as u8; 
		value >>= 7;
		result.push(byte);
	}
	result.reverse();

	let mut i = 0;
	while i < result.len()-1 && result[i] == 0 {
		i = i+1;
	};

	result = result[i..result.len()].to_vec();
	
	for i in 0..result.len()-1 {
		result[i] |= 0x80;
	}

	result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let sliced_values = slice_bytes(bytes);
    if sliced_values.is_err() {
    	return Err(sliced_values.unwrap_err());
    } else {
    	let mut result = vec![];
    	for bytes in sliced_values.unwrap() {
    		let value = bytes_value_to_single(bytes);
    		if value.is_err() {
    			return Err(value.unwrap_err());
    		} else {
    			result.push(value.unwrap());
    		}
    	}
    	return Ok(result);
    }
}

pub fn slice_bytes(bytes: &[u8]) -> Result<Vec<Vec<u8>>, Error> {
	let mut acc = vec![];
	let mut result = vec![];
	for byte in bytes {
		acc.push(*byte);
		if byte & 0x80 == 0 {
			result.push(acc);
			acc = vec![];
		}
	}
	if acc.len() > 0 {
		return Err(Error::IncompleteNumber);
	} else {
		return Ok(result);
	}
}

pub fn bytes_value_to_single(bytes: Vec<u8>) -> Result<u32, Error> {
	let mut res = String::new();
	let mut bytes_array : [u8; 8] = [0; 8];
	for (i, &byte) in bytes.iter().enumerate() {
		bytes_array[bytes.len()-1-i] = byte;	
	}
	let mut value = u64::from_le_bytes(bytes_array);
	for _i in 0..8 {
		let byte = (value & 0x7F) as u8;
		value >>= 8;
		let byte_str = format!("{:07b}", byte).chars().rev().collect::<String>(); 
		res.push_str(&byte_str);
	}
	let res = res.chars().rev().collect::<String>();
	let res = u64::from_str_radix(&res, 2).unwrap();
	if res > u32::max_value().into() {
		return Err(Error::Overflow);
	} else {
		return Ok(res as u32);
	}
}