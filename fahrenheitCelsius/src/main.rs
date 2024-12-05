use std::io;
fn main() {
    loop {
        
        let mut full_input = String::new();
        let mut trimmed_input: &str;

        loop {
            println!("Please enter f or c for if you want converted to fahrenheit or celsius: ");
            full_input = String::new();
            // Read a line of input
            io::stdin()
                .read_line(&mut full_input)
                .expect("Failed to read line");
            trimmed_input = full_input.trim();
            if trimmed_input == "f" || trimmed_input == "c" {
                break;
            }
        }
        
        println!("Please enter the degrees f or c you want converted: ");
        let mut numIn = String::new();
        // Read a line of input
        io::stdin()
            .read_line(&mut numIn)
            .expect("Failed to read line");
    
        let num: f32 = match numIn.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ERROR PARSING INPUT");
                continue
            }
        };
        
        if (trimmed_input == "f") {
            println!("Celsius value of {num} converted to {} fahrenheit", ctof(num));
        } else {
            println!("Fahrenheit value of {num} converted to {} celsius", ftoc(num));

        }

        
        // if ()


    }

}

fn ftoc(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0 
}

fn ctof(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}