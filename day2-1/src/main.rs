use std::io;

fn main() {
    let mut safe_cnt = 0i32;
    
    loop {
        let mut in_str = String::new();
        let num_read = io::stdin().read_line(&mut in_str)
            .expect("Failed to read line");

        if num_read == 0 {
            break;
        }

	let splits:Vec<&str> = in_str.split(" ").collect();

	let mut safe = true;
	let first: i32 =  splits[0].parse().expect("not an int");
	let second: i32 = splits[1].parse().expect("not an int");
	let len = splits.len();
	let dir = second > first;
	
	for (i, _el) in splits.iter().enumerate() {
	    if i + 1== len {
		break; // if we make it to here, row is safe
	    }
	    // trim them all even though only the last needs it
	    let x: i32 = splits[i].trim().parse().expect("not an int");
	    let y: i32 = splits[i+1].trim().parse().expect("not an int");
	    if dir {
		if (y > x) && (y <= x + 3) {
		    continue;
		}
	    } else {
		if (x > y) && (x <= y + 3) {
		    continue;
		}
	    }
	    // if we didn't continue, row is not safe
	    safe = false;
	    break; // abort row, it's not safe
	}

	if safe {
	    safe_cnt += 1;
	}

    }

    println!("{}", safe_cnt);
}
