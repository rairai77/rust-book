fn main() {
    let s = "Hello!"; // scope is main
    {
        let s = "Byebye!"; // scope is just in here
        println!("{s}"); // byebye!
    }
    println!("{s}"); // hello!
    
}
