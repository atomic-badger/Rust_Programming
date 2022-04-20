use std::io;
//ALLOWS FOR STRING VARIABLE I/O IN RUST


fn main() {
	
	//PROGRAM START
	println!("\nBEGINNING OF PROGRAM.\n");
	println!("\nThis program demonstrates basic input and output of a string variable in Rust.\n");
	
	//FIRST INPUT
	println!("\nPlease input a value:");
	
	let mut input_1 = String::new();
	
	io::stdin()
		.read_line(&mut input_1)
		.expect("\nERROR: FAILED TO READ LINE\n");
		
	println!("Your input was: {}\n", input_1);
	
	//SECOND INPUT
	println!("\nNow, let's input a second value.");
	
	let mut input_2 = String::new();
	
	io::stdin()
		.read_line(&mut input_2)
		.expect("\nERROR: FAILED TO READ LINE\n");
		
	println!("Your second input value was: {}\n", input_2);
	
	//PROGRAM FINISH
	println!("END OF PROGRAM.\n");

}
