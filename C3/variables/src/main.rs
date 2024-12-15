fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // limited operations you can do for a const
    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");
    let mut x = 5; // if we get rid of the mut then ... 
    println!("The value of x is {x}");
    x = 6; // ... this line would error
    println!("The value of x is: {x}");
    // but instead of using mut we could use shadowing
    let x = 3; // the thing being shadowed
    println!("The value of x is: {x}");
    let x = x+6; // the thing shadowing it
    println!("The value of x is: {x}");
    // there are some important differences though
    let spaces = "     ";
    let spaces = spaces.len(); // we re-imply the type here
    println!("{spaces}");
    // VS
    let mut spaces = "     ";
    // spaces = spaces.len(); // this one would error due to type conversion issues
    println!("{spaces}");
    spaces = "   ";
    println!("{spaces}");
}

