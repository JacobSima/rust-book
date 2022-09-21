fn main() {
    // Immutability
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6;        // cannot change this as it is immutable
    // println!("The value of x is: {x}");

    // Mutability
    let mut y = 5;
    println!("The value of x is: {y}");
    y = 10;
    println!("The value of x is: {y}");

    // Constant => forever immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The constant value is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let z = 5;
    let z = z + 1;
    {
      let z = z * 2;
      println!("The value of z in the inner scope is: {z}");   // 12
    }
    println!("The value of z is: {z}");   // 6
    
    // Can be able to declare variables with same name but different data-types
    let spaces = "  ";
    let spaces = spaces.len();
    println!("The spaces length is: {}", spaces);

    // Not shodowing in Mutable Type
    // let mut spaces_2 = "   ";
    // spaces_2 = spaces_2.len();     // This is an error.
    
    
}
