fn main() {
    println!("Hello, world!");
    another_function(500, 0.0);

    // let y = 6;
	//Error: you can't assign a statement to another variable
    // let x = (let y = 6); 

    // let x = 5;
    let y = {
    	let x = 3;
    	x + 1 // No ; indicates it is the return value
    };

    println!("Hello y: {}",y);

    println!("five {}", five());
    println!("five plus one {}", plus_one(five()));

}

fn plus_one(x: i32) -> i32{
 	x + 1
}

fn five() -> i32{
	5
}

fn another_function(x: i32, y: f64){
	println!("Another fun x: {}",x);
	println!("Another fun y: {}",y);
}
