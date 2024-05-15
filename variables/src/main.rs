fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

/*
------------------------------------------------------------------------------------
	Tuple: Fixed size (cannot grow), elements do not need to be same variable type
------------------------------------------------------------------------------------
		initialize:
		-	let tup: (i32, f64, u8) = (500, 6.4, 1);				-explicit
		-	let tup = (500, 6.4, 1);								-implicit
		access:
		-	let (x, y, z) = tup;									-pattern match using let
					or
		-	let five_hundred = tup.0;								-direct assignment of each element
			let six_point_four = tup.1;
			let one = tup.2;


------------------------------------------------------------------------------------
	Array: Fixed size (cannot grow), elements must all be the same variable type
------------------------------------------------------------------------------------
notes: -useful when size known and fixed (e.g. Months) - USE VECTOR IF YOU WANT TO GROW
		initialize:
		-	let a = [1, 2, 3, 4, 5];														-implicit
		-	let a: [i32; 5] = [1, 2, 3, 4, 5];												-explicit
		-	let a = [3; 5];																	-array of size 5, filled with 3
			-	let a = [3, 3, 3, 3, 3];
		-	let months = ["January", "February", "March", "April", "May", "June", "July",	-example
              "August", "September", "October", "November", "December"];
		access: (if accessing outside of range, will panic and crash)
		-	first = a[0];
		-	second = a[1];












------------------------------------------------------------------------------------
								Functions and Returns
------------------------------------------------------------------------------------
-	Return number five from a function: (fn foo() -> <return_value>)
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
-	! NOTE ! no ";" after 5. That is so it returns. Example:
	- Example:
	fn main() {
		let y = {
			let x = 3;
			x + 1
		};

		println!("The value of y is: {y}"); -> will be 4
	}
	-	Here we return 4 with x+1 (note no ";")

-	Last example:
	fn main() {
		let x = plus_one(5);

		println!("The value of x is: {x}");
	}

	fn plus_one(x: i32) -> i32 {
		x + 1
	}


-	USING IF IN LET STATEMENT
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
*/
