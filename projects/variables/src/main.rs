fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    //  数据基本类型转换
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    // 浮点型
    let a = 2.0;
    let b: f32 = 3.0;

    // 运算
    let sum = 5 + 10;
    let defference = 95.5 - 4.3;
    let product = 4 * 20;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("sum: {} {} {} {} {}",sum, defference, product, quotient, remainder);

    // 元祖类型
    let tup = (500, 6.4, 1);
    let (c, y, z) = tup;
    println!("tupL: {}", c);

    //  使用.  后面索引值
    let d: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = d.0;
    let six_point_four = d.1;
    let one = d.2;
    println!("d: {} {} {}", five_hundred, six_point_four, one);

}
