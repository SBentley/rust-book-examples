fn main() {
    // Scalar
    
    const CONST_NUMBER: u32 = 4_200;
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 7;
    println!("The value of x is {}", x);

    println!("The value of the const is {}",CONST_NUMBER);

    // Shaddow a variable
    let spaces = "    ";
    let spaces = spaces.len();

    println!("Spaces: {}", spaces);
    let cat = '😻';
    println!("{}", cat);
    let _b = true;

    // Compound
    
    // Tuple
    let tup: (i32, f64, u8) = (71, 2.89, 64);
    let (_x, y,_z) = tup;
    let small_int = tup.2;
    println!("The value of y is: {}", y);
    println!("The u8 in the tuple is: {}", small_int);
}
