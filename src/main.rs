use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("##########################");
    println!("## GUESS THE NUMBER !!! ##");
    println!("##########################");
    
    loop{
        println!("\nEnter a number: ");
        let secret_number = rand::thread_rng().gen_range(1..=10);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line...");
        
        let guess:i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Secret number: {secret_number}");
                println!("You win!!!");
                break;
            }
        }        
        println!("Secret number: {secret_number}");
    }    
    println!("***Game ends***");
}