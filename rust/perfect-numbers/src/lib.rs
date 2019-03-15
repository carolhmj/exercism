#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
	if num == 0 {
		return None;
	}
    let aliquot = aliquot_sum(num);
    if aliquot > num {
    	return Some(Classification::Abundant);
    } else if aliquot < num {
    	return Some(Classification::Deficient);
    } else {
    	return Some(Classification::Perfect);
    }
}

fn aliquot_sum(num: u64) -> u64 {
	(1..num).filter(|x| num % x == 0).sum()	
}

