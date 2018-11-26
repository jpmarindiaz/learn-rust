const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("x: {}", x);
    x = 6;
    println!("x: {}", x);

    // If not mutable need to use let to shadow
	let x1 = 5;
    let x1 = x1 + 1;
    let x1 = x1 * 2;
    println!("the value of x1 is {}",x1);
    println!("max points: {}", MAX_POINTS);

    // You can change the type of the variable with let
    // let creates a new variable
    let spaces = "   ";
	println!("spaces: {}",spaces);
	let spaces = spaces.len();
	println!("spaces len: {}",spaces);

	// // Error when trying to change the type of a mutable var
	// let mut spaces2 = "   ";
	// println!("spaces2: {}",spaces2);
	// spaces2 = spaces2.len();
	// println!("spaces2 len: {}",spaces2);

	

}
