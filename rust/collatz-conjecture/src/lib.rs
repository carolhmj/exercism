pub fn collatz(n: u64) -> Option<u64> {
    if n > 0 {
    	let mut steps : u64 = 0;
    	let mut n = n;
    	
    	while n != 1 {
    		println!("{:?}, {:?}", n, steps);
    		n = if n % 2 == 0 {
    			n/2 //even case
    		} else {
    			3*n+1 //odd case
    		};
    		steps += 1;	
    	}
    	Some(steps)
    } else {
    	None
    }
}
