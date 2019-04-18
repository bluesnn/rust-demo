fn main() {
    let number = 3;
    if number < 5 {
        println!("1");
    } else {
        println!("2");
    };

    let condition = true;

    let number = if condition {
        5
    } else {
        "1"
    };
    println!("number: {}", number);
}
