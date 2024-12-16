fn main() {
    println!("Hello, world!");
    another_function();
    some_function(3);
    labeled_measurement(3, "cows".to_string()); // note i have no idea if this is the right way to
                                                // do strings i just kinda did it
                                                // hope its fine
    let y = {
        let x=3;
        x+1 // note the lack of a semicolon that effectively 'returns' the value, it's called an
            // expression
    };

    println!("{y}");

    println!("{}", five());

    println!("{}", plus_one(3));

    println!("{}", returns_five());
}

fn another_function(){
    println!("Hello from inside another function");
}

fn some_function(x: i32){
    println!("x is {x}");
}

fn labeled_measurement(x: i32, h: String) {
    println!("{x} {h}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn returns_five() -> i32 {
    return 5; // note that no parenthesis are needed, but I think they're OK iyw
}
