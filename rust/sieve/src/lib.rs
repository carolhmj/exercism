use std::collections::BTreeSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let nonprimes = (2..upper_bound+1).into_iter()
    				.fold(BTreeSet::new(), |mut nonprimes, n| {
    					if !nonprimes.contains(&n) {
    						let mut i = 2;
    						while n * i <= upper_bound {
    							nonprimes.insert(n * i);
    							i += 1;	
    						}; 
    					};
    					nonprimes	
    				});
    (2..upper_bound+1).filter(|n| !nonprimes.contains(n)).collect::<Vec<u64>>()				
}
