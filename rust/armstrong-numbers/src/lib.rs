macro_rules! run {
    ($x:block until $y:expr) => {{
        while {
            $x;
            !$y
            } {}
    }};
    ($x:block if_still $y:expr) => {{
        while {
            $x;
            $y
            } {}
    }};
}

pub fn is_armstrong_number(num: u32) -> bool {
	let mut p = 1;
    let mut quot = 0;
    let mut dig = 0;
    let mut dec = 0;
    let mut num_mut = num;
    let mut digs = vec![];

    run!( {
    	quot = num_mut / 10_u32.pow(p);
    	dec = num_mut % 10_u32.pow(p);
    	dig = dec / 10_u32.pow(p-1);
    	p += 1;
    	num_mut -= dec;
    	digs.push(dig);
    } until quot == 0);
    
    return digs.iter().fold(0, |acc, d| acc + d.pow(digs.len() as u32)) == num
}
