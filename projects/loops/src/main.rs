fn main() {
    //Rust中提供三种循环：loop，while和for
    //01. loop无限循环，直到明确终止代码的执行
    //loop{
    //    println!("again!");
    //}

    let mut cnt = 0;

    let rst = loop{
        cnt += 1;

        if cnt == 10{
            break cnt*2;
        }
    };
    println!("The result of loop is {}", rst);

    //02. while条件循环
    let mut  number = 3;

    while number > 0{
        println!("number: {}", number);
        number -= 1;
    }

    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5{
        println!("{}", a[index]);
        index += 1;
    }

    //03. for
    for element  in a.iter(){
        println!("the value is {}", element);
    }
    
    for number in (1..4).rev(){
        println!("The number is {}", number);
    }

    println!("LIFTOFF");
}
