fn main() {
    let mut x = 5;
    println!("The value of x is: {:?}", x);

    x = 6;
    println!("The value of x is: {:?}", x); // When a variable is muttable you can change it, otherwise, you can't

    // Now we are going to see several types of variables and how do they work

    // Shadowing
    let var_0 = "Hello, I'm a string!";
    println!("This is my string that I created: {:?}", var_0);

    let var_0: u32 = 10;
    println!("Now, I have the same name as the var_0, but my value changed, even, changed my type! {:?}", var_0); // This is something
    // that you can't do with let mut variables

    // We can even make a few transformations to the variables in this way!
    let var_0: u32 = var_0 + 5;
    println!("Second shadow: {:?}", var_0);

    let var_0: u32 = var_0 * 3;
    println!("Third shadow: {:?}", var_0);

    // Types of numbers!
    // There are a lot of types, but we are going to focus on 2: Scalars and Compounds

    // Scalars: Integers, floats, booleans and characters

    // Integers
    let _integer_u32: u32 = 5; // Here is the default in Rust; These are the "Unsigned" numbers, which only allows positive numbers
    let _integer_i32: i32 = -23; // This is the "Signed" type of number, which allows negative numbers or with sign
    
    // Floats
    let _floats_f32: f32 = 4.32;
    let _floats_f64: f64 = 4.325643; // This one has more precision than f32 and its by default, this due to the modern CPUs
    
    // Booleans
    let _t = true; // Used in conditionals
    let _f = false;

    // Characters
    let _character_0 = 'C';  // Note: they also allows emojis, that's why they are called characters

    // Compound Variables: Tuples and Arrays

    // Tuples
    let tup: (u32, f64, bool) = (40, 42.09432, true);
    let _element_from_tuple = tup.0;

    // Arrays
    let _arr = [1, 3, 4, 6, 2, 0, 10];

}
