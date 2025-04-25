use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
  println!("Guessing Game!");
  
  let number = rand::thread_rng().gen_range(1..=10);
  
  loop {
    println!("Please input your guess between 1 and 10:");
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u8 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        print!("You must type a number! ");
        continue;
      }
    };

    println!("You guessed: {guess}");

    match guess.cmp(&number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}

