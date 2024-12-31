use std::io;
use regex::Regex;

fn main() {

    //    let re = Regex::new(r"mul\((?<m1>[0-9]+)\,(?<m2>[0-9]+)\)").unwrap();
    // matching "mul(123,456)"
    // here's the re in verbose mode
    let re = Regex::new(r"(?x)
      mul\(          # match mul(
      (?<m1>[0-9]+)  # match m1, an int
      \,             # match the comma
      (?<m2>[0-9]+)  # match m2, another int
      \)             # match the closing )
      ").unwrap();

    let mut prod = 0i32;
    
    loop {
        let mut in_str = String::new();
        let num_read = io::stdin().read_line(&mut in_str)
            .expect("Failed to read line");

        if num_read == 0 {
            break;
        }


	let hay = &in_str;

	// example code unpacks the re matches as a Vec of str's
	let prods: Vec<(&str, &str)> = re.captures_iter(hay).map(|caps| {
	    let m1 = caps.name("m1").unwrap().as_str();
	    let m2 = caps.name("m2").unwrap().as_str();
	    (m1, m2)
	}).collect();

	// convert the str's to i32's
	for v in prods.iter() {
	    let (s1, s2) = v;
	    let m1 = s1.parse::<i32>().unwrap();
	    let m2 = s2.parse::<i32>().unwrap();	
	    prod += m1*m2;
	}
    }
    println!("{}", prod);
}
