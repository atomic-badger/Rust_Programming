fn main() {

	//standard PRIMITIVE DATATYPES FOR RUST
	//VALUES CUT OFF AT u32 and i32 AS STANDARDS FOR SYSTEM
	
	// DECLARE AND ASSIGN VARIABLES
	let x1a :u8 = 255;					//unsigned int 8-bit
	let x2a :u16 = 256;					//unsigned int 16-bit
	let x3a :u32 = 4294967295;				//unsigned int 32-bit
	
	let y1a :i8 = -128;					//signed int 8-bit
	let y2a :i16 = -32768;					//signed int 16-bit
	let y3a :i32 = -2147483648;				//signed int 32-bit
	
	let a1a :f32 = 0.012345;				//6-place decimal precision;
	let b1a :f64 = 0.000000000000001;			//15-place decimal precision;	
	
	let answer_1 = 'Y';					//1-letter char type
	let answer_2 = 'N';					//also 1-letter char type
	
	let is_it_accurate_1 = true;				//Boolean Type							
	let is_it_accurate_2 = false;				//Boolean Type							
	


	// PRINT OUT VALUES
	println!("\nRUST DATATYPES.");
	println!("\nBEGINNING OF PROGRAM.\n");
	println!("\nThis program presents a few primitive datatype examples used in Rust.");
	println!("\tThese datatypes are also common in other systems languages like C, C++, and Java.");
	println!("\tNumeric values stop at 32 bits because of practical use in the originator's system.\n");
	
	println!("INT EXAMPLES, UNSIGNED:");
	println!("\tInt Value at u8 limit: {}", x1a);
	println!("\tInt Value at u16 limit: {}", x2a);
	println!("\tInt Value at u32 limit: {}", x3a);
	
	println!("\nINT EXAMPLES, SIGNED:");
	println!("\tInt Value at i8 limit: {}", y1a);
	println!("\tInt Value at i16 limit: {}", y2a);
	println!("\tInt Value at i32 limit: {}", y3a);
	
	println!("\nFLOAT AND DOUBLE EXAMPLES:");
	println!("\tFloat Value at 6 Digits:  {}", a1a);
	println!("\tFloat Value at 15 Digits:  {}", b1a);
	
	println!("\nCHAR EXAMPLES:");
	println!("\tFirst Letter Response: {}", answer_1);
	println!("\tSecond Letter Response: {}", answer_2);
	
	println!("\nBOOLEAN EXAMPLES:");
	println!("\tBoolean Response 1: {}", is_it_accurate_1);
	println!("\tBoolean Response 2: {}\n", is_it_accurate_2);
	
	println!("END OF PROGRAM.\n");
	
	//RETURN IS ASSUMED IN RUST AT END OF FILE LIKE IN C++
}
