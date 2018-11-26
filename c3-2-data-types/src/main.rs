fn main() {
	let sum = 5 + 10;
	println!("the value of sum is {}",sum);

	let quotient = 56.7 / 32.2;
	println!("the value of quotient is {}",quotient);
	
	let remainder = 5 % 3;
	println!("the value of remainder is {}", remainder);

	// // Error: need type definition for parse
	// let guess = "42".parse().expect("Not a number!");

	// let t = true;
    let f: bool = false; // with explicit type annotation
	println!("the value of f is {}", f);

    let c = 'c'; // single char
    let heart_eyed_cat = '😻';
    println!("the value of c is {} and cat is {}", c,heart_eyed_cat);

    
    // Compound types

    // "char array"



	let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y tup is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("x tup {} {} {}", five_hundred, six_point_four, one);

    // Arrays must be of the same type
    // Vectors are allowed to grow or shrink in size

    // Arrays have a type that indicate their size
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("a: {}",a);
    println!("a: {} {}", a[0], a[1]);

    let index = 10;
    let element = a[index];
	println!("element: {}", element); // runtime error   


}
