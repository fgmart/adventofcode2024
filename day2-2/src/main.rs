use std::io;

fn eval_line(nums: &Vec<i32>) -> bool {
    let first = nums[0];
    let second = nums[1];
    let len = nums.len();
    let dir = second > first;
    let mut safe = true;
	
    for (i, _el) in nums.iter().enumerate() {
	if i + 1 == len {
	    break;
	}

	let x = nums[i];
	let y = nums[i+1];
	
	if dir {
	    if (y > x) && (y <= x + 3) {
		continue;
	    }
	} else {
	    if (x > y) && (x <= y + 3) {
		continue;
	    }
	}

	safe = false;
	break;
    }

    return safe;
}


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

	let nums: Vec<i32> = splits.iter()
	    .map(|s| s.trim().parse::<i32>().unwrap()).collect();

	if eval_line(&nums) {
	    safe_cnt += 1;
	    continue;
	}

	// now try removing first element and see if it passes
	for (i, _el) in nums.iter().enumerate() {
	    let mut nnums = nums.clone();
	    nnums.remove(i);
	    if eval_line(&nnums) {
		safe_cnt += 1;
//		println!("decided {:?} is safe after removing elt {}", nnums, i);
		break;
	    }
	}
    }

    println!("{}", safe_cnt);
}
