# adventofcode2024
learning Rust!

## day1-1
OK so this was my first try at writing a Rust program.

Figuring out how to read from a file and parse ASCII to numbers took 90% of my time. 

Of course in doing that, I learned a lot.

I started trying to use some code I found to do the equivalent of scanf with a Regex. I never got that working - a combination of not fully understanding how to deal with ```Some``` values and I think the code was a little buggy.

Eventually I realized that just reading lines from ```stdin``` and using the ```expect``` method to handle the error case (by crashing with an error message) was the easiest thing.

Then using ```parse``` with ```expect``` was also easy.

I also discovered the variables shadowing each other, which is a cool feature, and it means that the ```x``` and ```y``` values in the input read-loop don't need to be mutable -- each new one just shadows the previous.

From the documentation for Vector, I encountered the idea of initializing a vector with ```1i32```. This was not explained properly. I figured out that this was the value ```1``` as an ```i32``` type. 

Cool. By this time, I'd figured out the :type approach to defining a variable type (Rust is strongly typed even though it can infer stuff) and I was able to write ```let mut xs: Vec<i32> = vec![];``` to get a Vector of ```i32```s with no initial values.

I Googled to find the ```zip``` form to go through the two vectors in parallel:

```
for (x, y) in xs.iter().zip(ys.iter()) {
        sum = sum + (x - y).abs();
    }
```

## day 1-2

This took just a couple minutes after the first puzzle was done :)

## day 2-1

A good part of the work was reading in the input. That seemed pretty clean at the end.

I'm not super happy with my logic - it's not functional at all. Lots of messy C-style control flow to see if we an get all the way through a row without failing, then it's safe.

```
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
```

I don't really like it.

## day 2-2

OK I put the for loop into to a fcn called ```eval_line```. That makes things a little bit better. But the messy control structure with ```continue```s and ```break```s is still there.

I really do like pure functional languages better... I don't understand the commenter who suggested Rust is 90% functional...

## day 3-1

This was a clear regex problem and the Rust example code for regex is pretty great https://docs.rs/regex/latest/regex/

I've defined the regex so that from an expr like ```mul(123,456)``` it extracts ```123``` as ```m1``` and ```456``` as ```m2```:
```
    let re = Regex::new(r"mul\((?<m1>[0-9]+)\,(?<m2>[0-9]+)\)").unwrap();
```
Then this block puts all the matches into a Vec of string tuples:
```
	let prods: Vec<(&str, &str)> = re.captures_iter(hay).map(|caps| {
	    let m1 = caps.name("m1").unwrap().as_str();
	    let m2 = caps.name("m2").unwrap().as_str();
	    (m1, m2)
	}).collect();
```
Then I iterate over the ```prods``` Vec converting from Str's to i32's, multiplying and summing up:
```
	for v in prods.iter() {
	    let (s1, s2) = v;
	    let m1 = s1.parse::<i32>().unwrap();
	    let m2 = s2.parse::<i32>().unwrap();	
	    prod += m1*m2;
	}
```
Pretty easy at the end.
