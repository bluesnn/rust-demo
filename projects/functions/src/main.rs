fn main() {

    // 函数调用，传参
    another_function(5, 6);
    println!("Hello, world!");

    // 表达式
    let b = {
        let a = 1;
        a + 1
    };
    println!("b: {}", b);

    // 返回值函数
    let c = five();
    println!("five: {}", c);

    // 定义参数类型和返回值类型 调用
    let e = plus_one(5);
    println!("plus: {}", e);
}

// 函数传参
fn another_function(x: i32, y: i32) {
    println!("Blue: {} {}", x, y);
}

// 返回值函数
fn five() -> i32 {
    5
}

// 定义参数类型和返回值类型
fn plus_one(d: i32) -> i32 {
    d + 1
}
