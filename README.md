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

