fn main() {

	let number = 5;

	if number < 5 {
    	println!("true condition");
	} else {
		println!("false condition");
	}

	let number = 12;
	if number % 4 == 0 {
		println!("number is divisible by 4");
	} else if number % 3 == 0 {
		println!("number is divisible by 3");
	} else if number % 2 == 0 {
		println!("number is divisible by 2");
	} else {
		println!("number is not divisible by 2,3 or 4.");
	}


	let condition = false;
	let number = if condition {
		5
	} else {
		6
	};
	println!("number is {}", number);

	// Error
	// let number = if condition {
	// 	5
	// } else {
	// 	"six"
	// };

	// LOOP

	// loop {
	// 	println!("again!");
	// }

	let mut counter = 0;

	let result = loop {
		counter += 1;

		if counter == 10{
			break counter * 2
		}
	};
	println!("result counter {}", result);
	assert_eq!(result, 20);


	// WHILE
	let mut number = 3;

	while number != 0 {
		println!("while number {}", number);
		number = number -1;
	}
	println!("lift off!!!");

	// FOR

	let a = [10, 20, 30, 40, 50];
	let mut index = 0;

	while index < 5 {
		println!("array while value is {}", a[index]);
		index += 1;
	}

	for element in a.iter() {
		println!("array for value is {}", element);
	}

	// count down

	for number in (1..4).rev() {
		println!("FOR countdown {}", number);
	}
	println!("lift off!!!");

}
