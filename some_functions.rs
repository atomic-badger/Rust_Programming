fn main() {

    banner_1();
    some_text();
    math_1();
    strings_1();
    banner_2();
}

fn banner_1() {
    println!("\nBEGINNING OF PROGRAM.\n");
}

fn some_text() {
    println!("\tThis program is intended to show some function examples.");
    println!("\tThe first function will be a few basic math problems.");
    println!("\tThe second function will use some string values.");
}

fn banner_2() {
    println!("\nEND OF PROGRAM.\n");
}

fn math_1() {

    println!("\nSOME BASIC MATHEMATICS USING OPERATORS:");

    //ADDITION
    println!("\nADDITION:");
    let num_x: i8 = 9;
    let num_y: i8 = 3;
    let result_1 = num_x + num_y;

    println!("\n\tThe Addition of num_x and num_y:");
    println!("\tnum_x = 9 and num_y = 3;");
    println!("\tAddition Result: \t\t\t {}\n", result_1);

    //SUBTRACTION
    println!("SUBTRACTION:");
    let num_x: i8 = 27;
    let num_y: i8 = 3;
    let result_1 = num_x - num_y;

    println!("\n\tThe Subtraction of num_x and num_y:");
    println!("\tnum_x = 27 and num_y = 3;");
    println!("\tSubtraction Result: \t\t\t {}\n", result_1);

    //MULTIPLICATION
    println!("MULTIPLICATION:");
    let num_x: i16 = 27;
    let num_y: i16 = 3;
    let num_z: i16  = 2;
    let result_1 = num_x * num_y * num_z;

    println!("\n\tThe Multiplication of num_x, num_y, and num_z:");
    println!("\tnum_x = 27, num_y = 3, and num_z = 2;");
	println!("\tMultiplication Result: \t\t\t {}\n", result_1);

    //DIVISION
    println!("DIVISION:");
    let num_x: i16 = 27;
    let num_y: i16 = 3;
    let result_1 = num_x / num_y;

    println!("\n\tThe Division of num_x and num_y:");
    println!("\tnum_x = 27 and num_y = 3;");
    println!("\tDivision Result: \t\t\t {}\n", result_1);

    //REMAINDER
    println!("MODULUS:");
    let num_x: i16 = 29;
    let num_y: i16 = 2;
    let result_1 = num_x % num_y;

    println!("\n\tThe Remainder of num_x and num_y:");
    println!("\tnum_x = 29 and num_y = 2;");
    println!("\tModulus Result: \t\t\t {}\n", result_1);

    println!("\nEND OF MATHEMATICS SECTION.\n");
}

fn strings_1() {

    println!("\nSOME BASIC STRINGS IN RUST:");

	let my_name = String::from("Dwayne Brock");
	let my_age: u32 = 54;
	let my_color = String::from("Red");
	let quest = String::from("Holy Grail");

	println!("\n\tName: {}", my_name);
	println!("\tAge: {}", my_age);
	println!("\tQuest: {}", quest);
	println!("\tFavorite Color: {}\n", my_color);
}
