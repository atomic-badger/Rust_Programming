fn main() {

	// EXAMPLE OF BASIC STRINGS

	let my_name = String::from("Dwayne Brock");
	let my_age: u32 = 54;
	let my_color = String::from("Red");
	let quest = String::from("Holy Grail");
	
	let string ="BEGINNING OF PROGRAM.";
	println!("\n{}", &string[0..21]);
	
	println!("\nName: {}", my_name);
	println!("Age: {}", my_age);
	println!("Quest: {}", quest);
	println!("Favorite Color: {}\n", my_color);
	
	let string ="END OF PROGRAM.";
	println!("{}\n", &string[0..15]);
	
	return;
}
