fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    println!("five = {}.", five());
    println!("plus_on = {}", plus_one(6));
}

fn another_function(x:i32, y:i32){
    println!("x = {}, y = {}.", x, y);
}

fn five()->i32{
    5
}

fn plus_one(x:i32)->i32{
    x+1
}
