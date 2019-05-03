use std::u64::MAX;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    //i'm assuming the input is in row-major format
    let m = input.len();
 
    let biggest_row : Vec<Vec<(usize, &u64)>> = input.iter().map(|row| {
    	row.iter()
    	   .enumerate()
    	   .filter(|&(_, item)| item == row.iter().max().unwrap())
    	   .collect::<Vec<_>>()
    }).collect();
    
    let saddle_points : Vec<(usize, usize)> = biggest_row.into_iter()
    	.enumerate()
    	.map(|(i, v_jmax)| {
    		v_jmax.iter().map(|&(j, max)| {
    			let smallest = find_smallest_column(j, input);
    			if smallest == *max {
    				Some((i,j))
    			} else {
    				None
    			}
    		}).collect::<Vec<_>>()		
    	}).flatten().filter(|optsaddle| {
    		optsaddle.is_some()
    	}).map(|optsaddle| {
    		optsaddle.unwrap()
    	}).collect();

    return saddle_points;
}

pub fn find_smallest_column(col: usize, input: &[Vec<u64>]) -> u64 {
	let mut smallest = MAX;
	for i in 0..input.len() {
		let v = input[i][col];
		if v < smallest {
			smallest = v;
		}
	}
	return smallest; 	
}