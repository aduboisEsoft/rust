use std::io;

fn main() {
    
    let mut value: f64;

    loop {
        println!("Choose way of conversion");
        println!("1 -> Celsius to Farhenheit");
        println!("2 -> Farhenheit to Celsius");

        let mut choice = String::new(); 

        io::stdin()
            .read_line(&mut choice)
            .expect("Fail to read line");
        
        let choice: u8 = match choice.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        }; 

        if choice != 1 && choice != 2 {
            println!("This choice doesn't exist");
            continue;
        }
        
        value = ask_value_to_convert();
        value = if choice == 1 {celsius_to_farhenheit(value)} else {farhenheit_to_celsius(value)};
        break;
    }

    println!("Converted value is {value}");
}

fn celsius_to_farhenheit (value: f64) -> f64 {
    value * 1.8 + 32.0
}

fn farhenheit_to_celsius (value: f64) -> f64 {
    (value - 32.0) / 1.8
}

fn ask_value_to_convert() -> f64{
  loop {
        println!("Enter the value to convert :");
        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Fail to read line");

        match value.trim().parse(){
            Ok(num) => break num,
            Err(_) => continue,
        };
    }
} 
