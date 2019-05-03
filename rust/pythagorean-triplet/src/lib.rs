pub fn find() -> Option<u32> {
	const PYTHAGOREAN_SUM : u32 = 1000;

    //we'll let a assume any value between 1 and 1000, and fix b and c based on its value
    for a in 0..PYTHAGOREAN_SUM+1 { 
      
      //we'll limit b to combinations not covered by the previous loops 
      for b in a..PYTHAGOREAN_SUM-a+1 { 
        
        //now we only have one choice on c
        let c = PYTHAGOREAN_SUM - a - b; 
        
        //now we see if all values are different, and a^2 + b^2 = c^2
        if all_different(a,b,c) && a.pow(2) + b.pow(2) == c.pow(2) {
        	return Some(a*b*c)
        }
      }
    }

    //if we didn't find the values, just return none
    None
}

fn all_different(a : u32, b : u32, c : u32) -> bool {
	a != b && b != c && a != c
}
