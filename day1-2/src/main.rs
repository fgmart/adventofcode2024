use std::io;

fn main() {
    let mut xs: Vec<i32> = vec![];
    let mut ys: Vec<i32> = vec![];
    
    loop {
        let mut in_str = String::new();
        let num_read = io::stdin().read_line(&mut in_str)
            .expect("Failed to read line");

        if num_read == 0 {
            break;
        }

        let mut split = in_str.split("   ");

        let x: i32 = split.next().expect("first split failed")
            .parse().expect("first not an int");
        xs.push(x);


        let y: i32 = split.next().expect("second split failed").trim()
            .parse().expect("second not an int");
        ys.push(y);

    }

    xs.sort();
    ys.sort();

    let mut sum = 0i32;
    
    for x in xs.iter() {
	for y in ys.iter() {
	    if (x == y) {
		sum = sum + x;
	    }
	}
    }

    println!("{}", sum);
}
