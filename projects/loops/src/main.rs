fn main() {
    let mut counter = 0;

    //  loop循环，break退出
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    // while循环
    let mut number = 3;

    while number != 0 {
        println!("number: {}", number);
        number = number - 1;
    }
    println!("LIFT: {}", number);

    // for遍历
    let arr = [10, 20, 30, 50, 60, 70];
    let mut index = 0;

    // while遍历数组此方法的缺陷，一旦数组长度小于条件数值，就会发生错误
    while index < 5 {
        println!("value: {} {}", arr[index], index);
        index += 1;
    }

    for element in arr.iter() {
        println!("then value: {}", element);
    }

    for for_rev in (1..4).rev() {
        println!("for_rev: {}", for_rev);
    }
    println!("LIFTOFF!");
}
