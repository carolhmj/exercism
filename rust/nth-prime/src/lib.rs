pub fn nth(n: u32) -> u32 {
	//tells us what is the index of our most recently found prime
    let mut current_prime_index = 0;
    //since we know the first prime is 2, we'll start with it
    let mut number = 2; 
    while current_prime_index < n {
    	number = number+1;
    	if is_prime(number) {
    		current_prime_index = current_prime_index+1;
    	}
    }
    number
}

//n is prime iff its only divisors are 1 and itself
//we'll iterate over numbers between 2 and itself, if any number in this
//range is a divisor of n, then we know it isn't prime
fn is_prime(n: u32) -> bool {	
	for pot_divisor in 2..n {
		if is_divisor_of(pot_divisor, n) {
			return false
		}
	}
	true
}

//tells us if a is a divisor of b
fn is_divisor_of(a: u32, b: u32) -> bool {
	b % a == 0
}