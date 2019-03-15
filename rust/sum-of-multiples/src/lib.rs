pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    for n in 0..limit {
    	if is_multiple(n, factors) {
    		sum += n;
    	}
    }
    sum
}

fn is_multiple(number : u32, factors: &[u32]) -> bool {
	factors.iter().any(|&x| number % x == 0)	
}
