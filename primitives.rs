//PRIMITIVE DATATYPES

fn main() {

    //HEADER
    println!("\nPRIMITIVE DATATYPES IN RUST:\n");

    //UNSIGNED INTEGER DATATYPES
    println!("\nUnsigned Integers:");
    println!("\tTYPE:\t\tRANGE:\t\t\t\t\t\t\t\t\t\t\tEXAMPLE:");
    println!("\tu8:\t\t0 to 2^8 or 0 to 255\t\t\t\t\t\t\t\t\tlet var_x1 :u8 = 9;");
    println!("\tu16:\t\t0 to 2^16 - 1 or 0 to 65,535\t\t\t\t\t\t\t\tlet var_x2 :u16 = 10000;");
    println!("\tu32:\t\t0 to 2^32 - 1 or 0 to 4,294,967,295\t\t\t\t\t\t\tlet var_x3 :u32 = 3000000000;");
    println!("\tu64:\t\t0 to 2^64 - 1 or 0 to 18,446,744,073,709,551,615\t\t\t\t\tlet var_x4 :u64 = 10000000000;");
    println!("\tu128:\t\t0 to 2^128 - 1 or 0 to approximately 3.14X10^38");
    println!("\tusize:\t\t0 to either 2^32 - 1 or 2^64 - 1\n");

    //SIGNED INTEGER DATATYPES
    println!("\nSigned Integers:");
    println!("\tTYPE:\t\tRANGE:\t\t\t\t\t\t\t\t\t\t\tEXAMPLE:");
    println!("\ti8:\t\t-2^7 to 2^7 - 1 or -128 to 127\t\t\t\t\t\t\t\tlet var_y1 :i8 = 0;");
    println!("\ti16:\t\t-2^15 to 2^15 - 1 or -32,768 to 32,767\t\t\t\t\t\t\tlet var_y2 :i16 = 30000;");
    println!("\ti32:\t\t-2^31 to 2^31 - 1 or -2,147,483,648 to 2,147,483,647\t\t\t\t\tlet var_y3 :i32 = 2000000000;");
    println!("\ti64:\t\t-2^63 to 2^63 -1 or -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807\t\tlet var_y4 :i64 = 10000000000;");
    println!("\ti128:\t\t-2^127 to 2^127 -1 (roughly -1.7X10^38 to +1.7X10^38)");
    println!("\tisize:\t\tEither -2^31 to 2^31 - 1 or -2^63 to 2^63 - 1\n");

    //FLOATING POINT TYPES
    println!("Floating Point Numeric Datatypes:");
    println!("\tTYPE:\t\tPRECISION:\t\t\t\t\t\t\t\t\t\tRANGE:");
    println!("\tf32:\t\tIEEE single-precision (at least 6 decimal digits)\t\t\t\t\tRoughly -3.4 X 10^38 to +3.4 X 10^38");
    println!("\tf64:\t\tIEEE double-precision (at least 15 decimal digits)\t\t\t\t\tRoughly -1.8 X 10^308 to +1.8 X 10^308\n");




}
