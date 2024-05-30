fn main() {
    println!("Hello, world!");

    let mut counter = 1;

    let stop_loop = loop {
      counter *= 2;
      if counter > 100 {
        break counter;
      }
    };

    println!("Break the loop at counter = {}", stop_loop);

    let mut couter2 = 1;

    while couter2 < 5 {
      println!("We loop a while...");
      couter2 = couter2 + 1;
    }

    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
      println!("The {} is a big bird.", bird);
    }

    for number in 0..5 { // 범위 표기법
      println!("{}", number * 2);
    }
}

