fn main() {

//SIMPLE MATH OPERATORS IN RUST: ADD, SUBTRACT, MULTIPLY, DIVIDE, REMAINDER
// USES THE +, -, *, /, AND % OPERATORS

	println!("\nBEGINNING OF PROGRAM.");
	println!("\nThis program presents some simple arithmetic completed in Rust.\n");
	
	//ADDITION
    let num_x: i8 = 9;
    let num_y: i8 = 3;
	let result_1 = num_x + num_y;
    
	println!("\nThe Addition of num_x and num_y:");
	println!("\tnum_x = 9 and num_y = 3;");
	println!("\t\t\tAddition Result: \t\t {}", result_1);
	
	//SUBTRACTION
    let num_x: i8 = 27;
    let num_y: i8 = 3;
 	let result_1 = num_x - num_y;
 	
 	println!("\nThe Subtraction of num_x and num_y:");
 	println!("\tnum_x = 27 and num_y = 3;");
	println!("\t\t\tSubtraction Result: \t\t {}", result_1);
	
	//MULTIPLICATION
	let num_x: i16 = 27;
    let num_y: i16 = 3;
    let num_z: i16  = 2;
    let result_1 = num_x * num_y * num_z;
    
    println!("\nThe Multiplication of num_x, num_y, and num_z:");
    println!("\tnum_x = 27, num_y = 3, and num_z = 2;");
	println!("\t\t\tMultiplication Result: \t\t {}", result_1);
	
	//DIVISION
	let num_x: i16 = 27;
    let num_y: i16 = 3;
    let result_1 = num_x / num_y;
    
    println!("\nThe Division of num_x and num_y:");
    println!("\tnum_x = 27 and num_y = 3;");
	println!("\t\t\tDivision Result: \t\t {}", result_1);
	
	//REMAINDER
	let num_x: i16 = 29;
    let num_y: i16 = 2;
    let result_1 = num_x % num_y;
    
    println!("\nThe Remainder of num_x and num_y:");
    println!("\tnum_x = 29 and num_y = 2;");
    println!("\t\t\tModulus Result: \t\t {}", result_1);
	
	println!("\nEND OF PROGRAM.\n");

}
