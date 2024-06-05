extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng()
    .gen_range(1..100); // 1~100을 의미함. 상한선을 제외됨.

  // println!("The secret number is : {}", secret_number);

  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess
      .trim() // io에서 Enter를 입력하므로 개행문자 제거
      .parse() {  // 문자열에서 숫자로 파싱
        Ok(num) => num,
        // 숫자가 아닌 값을 입력하면 종료가 아닌 계속 입력할 수 있게함
        Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }
}
