use std::io;
fn main() {
    'main: loop {
        println!("Enter whether you wish to input a temp in C or F");
        let mut mode = String::new();
        match io::stdin().read_line(&mut mode) {
            Ok(_) => {}
            Err(_) => continue,
        }
        let mode: bool = match mode.trim().to_lowercase() {
            val if val == "F".to_lowercase() => true,
            val if val == "C".to_lowercase() => false,
            val if val == "Q".to_lowercase() => break 'main,
            _ => continue,
        };
        println!("Enter a number you wish to convert:");
        let mut number = String::new();
        match io::stdin().read_line(&mut number) {
            Ok(_) => {}
            Err(_) => continue,
        }
        let number: f32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if number.trim() == "q" {
                    break 'main;
                } else {
                    continue;
                }
            }
        };
        if mode {
            println!("{}", (number - 32.0) * 5.0 / 9.0);
        } else {
            println!("{}", number * 9.0 / 5.0 + 32.0);
        }
    }
}
