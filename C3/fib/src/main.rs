use std::io;
fn main() {
    'main: loop {
        let mut number = String::new();
        print!("> ");
        match io::stdin().read_line(&mut number) {
            Ok(_) => {}
            Err(_) => continue 'main,
        };
        let number: i32 = match number.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                if number.trim() == "q" {
                    break 'main;
                } else {
                    continue 'main;
                }
            }
        };
        if number == 0 {
            println!("0")
        } else if number == 1 {
            println!("1")
        } else {
            let number = number - 1;
            let mut a = 0;
            let mut b = 1;
            for _ in 0..number {
                (a, b) = (b, a + b);
            }
            println!("The {}th fibonacci number is: {b}", number+1);
        }
    }
}
