use std::cmp::Ord;

pub fn find<R, T>(array: R, key: T) -> Option<usize>
where T : Ord, R: AsRef<[T]> {
	let array = array.as_ref();
	if array.len() > 0 {
    	find_in_range(array, key, 0, array.len()-1)
    } else {
    	None
    }
}

pub fn find_in_range<T>(array: &[T], key: T, low: usize, high: usize) -> Option<usize> 
where T : Ord {
	let mid = (low + high) / 2;
	if array[mid] == key {
		Some(mid)
	}  else if low > high || high - low <= 1 {
		if array[high] == key {
			Some(high)
		} else {
			None
		}
	} else if array[mid] < key {
		find_in_range(array, key, mid, high)
	} else {
		find_in_range(array, key, low, mid)
	}
}