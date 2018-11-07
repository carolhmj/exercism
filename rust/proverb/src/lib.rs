pub fn build_proverb(list: Vec<&str>) -> String {
    let mut proverb = String::new();
    for line_number in 0..list.len() {
    	if last_line(line_number, list.len()) {
    		proverb.push_str(&format!("And all for the want of a {}.", 
    			list[0]));
    	} else {
    		proverb.push_str(&format!("For want of a {} the \
    			{} was lost.\n", list[line_number], list[line_number+1]));
    	}
    }
    proverb
}

fn last_line(line_number : usize, total_lines : usize) -> bool {
	line_number == total_lines-1
}
