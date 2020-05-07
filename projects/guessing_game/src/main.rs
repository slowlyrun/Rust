use std::io;
use std::cmp::Ordering;
use rand::Rng;

//创建一个猜猜看游戏
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is {}", secret_number);
    
    loop{
        println!("Guess the number!");
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

        println!("Your guess is {}", guess.trim());

        let guess:u32 =  match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
           Ordering::Less    => println!("Too Small."),
           Ordering::Greater => println!("Too big."),
           Ordering::Equal   => {
               println!("You win.");
               break;
           },
       }
    }
}
