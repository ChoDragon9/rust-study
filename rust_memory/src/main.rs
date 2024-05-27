fn main() {
  // { // mascot의 범위
  //   let mascot = String::from("ferris");
  // }
  // println!("{}", mascot); // not found in this scope
  {
    let mascot = String::from("ferris");
    let ferris = mascot; // mascot의 소유권은 ferris로 이동됨.

    // println!("{}", mascot); // mascot은 더이상 사용할 수 없음
    println!("{}", ferris);
  }

  // 참조를 사용하면 소유권을 차지하지 않고 값을 "대여"할 수 있음
  let mut greeting = String::from("hello");
  change(&mut greeting);
  print_greeting(&greeting);
  print_greeting(&greeting);

  // 대여 및 변경 가능 참조는 '하나'만 할 수 있음
  // let mut val = String::from("val");
  // let ref1 = &val;
  // let ref2 = &mut val;
  // println!("{}, {}", ref1, ref2);

  // 참조를 해도 수명이 끝나는 곳에서 사용못함
  // let x;
  // {
  //   let y = 42;
  //   x = &y;
  // }
  // println!("x: {}", x); // y의 수명이 짧으므로 에러 발생

  let magic1 = String::from("abracadabra");
  let result;
  {
    let magic2 = String::from("shazam");
    result = longest_word(&magic1, &magic2);
  }

  // magic2의 수명은 해당 위치에서 끝나므로 에러 발생함
  // println!("The longest magic word is {}", result);

  #[derive(Debug)]
  struct Hightlight<'document>(&'document str);

  let text = String::from("0123456789");
  let fox = Hightlight(&text[0..5]);
  let dog = Hightlight(&text[5..10]);
  let erase = text;
  println!("{:?}, {:?}", fox, dog);
}

// x, y의 수명을 알 수 없으므로 수동으로 수명에 주석을 달아야함
// 수명은 제네릭 수명 매개 변수를 선언하고 매개 변수 목록과 함수 이름 사이에 선언을 추가해야함
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn print_greeting(message: &String) {
  println!("Greeting {}", message);
}

fn change(message: &mut String) {
  message.push_str(", world");
}

// 소유권
// fn process(input: String) {}

// fn caller() {
//   let s = String::from("Hello, world!");
//   process(s); // s의 소유권이 process로 이동됨
//   process(s);
// }

// 값을 복사하는 형태
// fn process(input: u32) {}

// fn caller() {
//   let n = 1u32; // 값을 복사함
//   process(n);
//   process(n);
// }