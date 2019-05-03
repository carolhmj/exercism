pub fn raindrops(n: u32) -> String {
    let mut speak = String::new();

    if n % 3 == 0 { 
    	speak.push_str("Pling");
    }
    if n % 5 == 0 {
    	speak.push_str("Plang");
    }
    if n % 7 == 0 {
    	speak.push_str("Plong");
    }

    if speak.len() == 0 {
    	speak.push_str(&n.to_string());
    }

    return speak
}
