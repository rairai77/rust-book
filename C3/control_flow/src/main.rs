use std::io;

fn main() {
    loop {
        let mut number = String::new();

        match io::stdin().read_line(&mut number) {
            Ok(_) => {}
            Err(_) => continue,
        }

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if number.trim() == "q" {
                    break;
                } else {
                    continue;
                }
            }
        };

        if number < 5 {
            println!("number is less than 5");
        } else {
            println!("number not less than 5 :(");
        }
        if number != 0 {
            // note that unlike many langauges, rust will NOT type coerce to bool, so
            // i cant just do if number
            println!("nonzero number");
        }
        if number % 3 == 0 {
            println!("divisible by 3");
        } else if number % 2 == 0 {
            println!("number divisible by 2");
        } else {
            println!("idk what its divisible by but its not 2 or 3");
        }
    }
    let mut count = 0;
    let mut count2 = 0;
    // labels to break out of loop
    'outer_loop: loop {
        count2 += 1;
        loop {
            count += 1;
            if count2 == 3 {
                break 'outer_loop;
            } else if count >= 3 {
                break;
            }
        }
    }
    let mut number = 3;

    // while loop just like normal
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    for i in (1..4).rev() { // literally almost exactly like python for each
        println!("{i}!");
    }
    println!("LIFTOFF!!!");

}
