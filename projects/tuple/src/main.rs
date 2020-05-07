fn main() {
    //Rust 中元组的语法
    let tup:(i32, f64, u8) = (500, 3.4, 1);

    let (x, y, z) = tup;//模式匹配,解构
    println!("The value of y is {}", y);

    let x = tup.0;//索引
    let y = tup.1;
    let z = tup.2;
    println!("The value of z is {}", z);
}
