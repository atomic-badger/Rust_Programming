fn main() {

//ARRAYS EXAMPLE IN RUST

	println!("\nBEGINNING OF PROGRAM.\n");
	println!("\nPrinting out Arrays:");
	println!("\tAn array is a data structure.");
	println!("\tAn array holds values of the same data type.");
	println!("\tHere is an example array.\n");
	
	let number_holder:[i32;6] = [1,9,18,36,72,999];
	
	println!("List of values:\t\t {:?}", number_holder);
	
	println!("\nThe First Number is:\t{}", number_holder[0]);
	println!("The Second Number is:\t{}", number_holder[1]);
	println!("The Third Number is:\t{}", number_holder[2]);
	println!("The Fourth Number is:\t{}", number_holder[3]);
	println!("The Fifth Number is:\t{}", number_holder[4]);
	println!("The Sixth Number is:\t{}\n", number_holder[5]);
	
	println!("\nEND OF PROGRAM.\n");
	
	
	return;
}
