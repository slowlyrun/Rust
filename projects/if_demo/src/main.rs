fn main() {
    println!("Hello, world!");
    let num = 3;

    if num > 5{
        println!("Condition was true.");
    }else{
        println!("Condition was false.");
    }

    let num = 5;
    if num % 3 == 0{
        println!("Number is divisible by 3.");
    }else if num % 4 == 0{
        println!("Number is divisible by 4");
    }else if num % 5 == 0{
        println!("Number is divisible by 5");
    }else{
        println!("Number is not divisible by 3, 4, 5.");
    }

    let condition = true;
    let number = if condition{
                  5
                 }else{
                 4
                 };
    println!("The number is:{}", number);
}
