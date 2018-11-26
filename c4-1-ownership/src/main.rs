fn main() {

	// let s = "hello"; // string literal

    let s0 = String::from("hello");
	println!("Hello, world! {}",s0);

    let mut s = String::from("hello"); // can be mutable
    s.push_str(", world");
	println!("Hello, world?:    {}",s);


	// y makes a copy, and both x and y are 5.
	// This happens because integers are simple types.
	let x = 5;
	let y = x;
	println!("x?: {}",x);
	println!("y?: {}",y);
	
	let s1 = String::from("hello");
	let s2 = s1;
	// println!("s1?: {}", s1); // Error -> moved value
	println!("s2?: {}", s2);

	let s1a = String::from("hello");
	let s2a = s1a.clone();

	println!("s1a = {}, s2a = {}", s1a, s2a);


	// Ownership in functions

	let s = String::from("hello");
	takes_ownership(s);
	// println!("s after ownership {}", s); // Cannot call s

	let x = 5;
	makes_copy(x);
	println!("X after ownership {}... ok", x); 


	let s1_ = gives_ownership();
	let s2_ = String::from("hello"); // Need to return the value to be used later
	let s3_ = takes_and_gives_back(s2_);

	// Return multiples values in a tuple

	let s1__ = String::from("hello");
	let (s2__, len) = calculate_len(s1__);
	println!("length of {} is ... {}", s2__, len);


}

////
fn calculate_len(s:String) -> (String, usize){
	let length = s.len();
	(s, length)
}


////

fn gives_ownership() -> String {
	let some_string = String::from("hello gives");
	some_string
}

fn takes_and_gives_back(a_string: String) -> String{
	a_string
}

////

fn takes_ownership(some_string: String){
	println!("fun {}", some_string);
}

fn makes_copy(some_integer: i32){
	println!("fun {}", some_integer);
}



