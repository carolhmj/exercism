pub fn verse(n: i32) -> String {
    if n >= 1 && n <= 99 {
      format!("{} on the wall, {}.\nTake {} down \
        and pass it around, {} on the wall.\n", 
        bottles(n), bottles(n), take_down(n), bottles(n-1))
    } else if n == 0 {
      format!("No more bottles of beer on the wall, \
      	no more bottles of beer.\nGo to the store and buy some more, \
      	99 bottles of beer on the wall.\n")
    } else {
      "I don't know how to sing to this amount of bottles!\n".to_string()
    }
}

fn bottles(n : i32) -> String {
  if n > 1 {
  	format!("{} bottles of beer", n)  
  } else if n == 1 {
  	format!("{} bottle of beer", n)
  } else if n == 0 {
  	"no more bottles of beer".to_string()
  } else {
    format!("I'm sorry, I don't know how to sing to this")
  }
}

fn take_down(n : i32) -> String {
	if n > 1 {
		"one".to_string()
	} else {
		"it".to_string()
	}
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();
    for n in (end..start+1).rev() {
    	song.push_str(&verse(n));
    	if n > end {
    		song.push('\n');
    	}		
    }
    song
}