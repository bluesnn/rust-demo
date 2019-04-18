use std::io;

fn main() {
    println!("Celsius converted to Fahrenheit!");
    println!("Please input celsius.");

    loop {
        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = temperature.trim().parse()
            .expect("type a number!");

        let plus: f32 = 33.8;

        let run = temperature * plus;

        println!("temperature: {}", run);
    }

}

// fn plus() -> f32 {
//     33.8
// }
