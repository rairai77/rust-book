use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Readline failed");
    let index: usize = index.trim().parse().expect("please enter an uint");
    println!("{}", a[index]); // i don't think you can put an expression inside of {}
                             
    // key takeaway here is that rust will runtime
    // exception if you do index out of bounds unlike C
    // also the syntax let a = [3; 5] for let a = [3,3,3] is pretty cool
    // also to specify types for array its like let a [type; length] =
    // [boop boop things and stuff]
}
