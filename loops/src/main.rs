// --------------------------------------------------------------
//	-	THIS DOCUMENT EXPLORES LOOPS IN RUST
//	-	COMMENTS ABOVE CODE ARE ABOUT NOTEWORTHY CONCEPTS/LINES
// --------------------------------------------------------------


// in this case let result = [...] is an assignment. Hence the ";" that still returns 20.
// like let mut counter = 0;
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}


// --------------------------------------------------------------
// 						LOOP LABELS	
// --------------------------------------------------------------
// 	-	You can define a Label before a loop using <'>
// 	-	e.g. '<label>: loop {}
// 	-	You can then use this label to break or continue the outer loop from an inner loop
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
//	-	Here the first break breaks out of the inner loop
//	-	The second break refers to the outer loop label, breaking out of the outer loop


// --------------------------------------------------------------
// 						CONDITIONAL LOOPS
// --------------------------------------------------------------
//	-	Same as C/C++. while loops.
fn main() {
    let mut number = 3;

    while (number != 0) {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

//------------------------
//		FOR LOOPS
//------------------------
//	-	!! Note !! parenthesis around (element in a) makes this not compile !!
//	-	error[E0425]: cannot find value `element` in this scope
//   --> src/main.rs:69:34
//   |
// 69 |         println!("the value is: {element}");
//   |                                  ^^^^^^^ not found in this scope
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}