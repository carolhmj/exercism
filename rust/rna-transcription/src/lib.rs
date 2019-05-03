#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);
	
impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        match dna.chars().position(|c| !['A', 'C', 'G', 'T'].contains(&c)) {
        	Some(index) => Err(index),
        	None => Ok(DNA(dna.to_string())),
        }
    }

    pub fn into_rna(self) -> RNA {
        RNA(self.0.chars()
        	.map(|c| match c {
        		'G' => 'C',
        		'C' => 'G',
        		'T' => 'A',
        		'A' => 'U',
        		 _ => panic!("DNA with an anomalous sequence"),
        	})
        	.collect::<String>())
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        match rna.chars().position(|c| !['A', 'C', 'G', 'U'].contains(&c)) {
        	Some(index) => Err(index),
        	None => Ok(RNA(rna.to_string())),
        }
    }
}
