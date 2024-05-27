fn main() {
  // 일반적으로 프로그램이 복구할 수 없는 상태에 도달할 때 panic!를 사용한다.
  // 오류에서 복구할 방법이 전혀 없는 상태다.
  // panic!("Farewell!")

  // let v = vec![0, 1, 2, 3];
  // println!("{}", v[6])

  // Rust 표준 라이브러리는 값이 없을 가능성이 있을 때 사용되는 Option<T> 열거형을 제공한다.
  // Option<T>은 Rust 코드에서 널리 사용되며 존재하거나비어 있을 수 있는 값을 사용할 때 유용하다.
  // 다른 많은 언어에서 값이 없는 경우 null 또는 nil을 사용하여 모델링되지만 Rust는 null을 사용하지 않는다.
  // Rust는 선택적 문자열을 모델링하려면 Option<String>으로 명시적으로 래핑해야 한다.
  // enum Option<T> {
  //   None,
  //   Some(T)
  // }

  let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

  let first = fruits.get(0);
  println!("{:?}", first);

  let third = fruits.get(2);
  println!("{:?}", third);

  let non_existent = fruits.get(99);
  println!("{:?}", non_existent);

  // 패턴 일치

  for &index in [0, 2, 99].iter() {
    match fruits.get(index) {
      Some(fruits_name) => println!("It's a delicious {}!", fruits_name),
      None => println!("There is no fruit! :("),
    }
  }

  for &index in [0, 2, 99].iter() {
    match fruits.get(index) {
      Some(&"coconut") => println!("Coconuts are awesome!"),
      Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
      None => println!("There is no fruit! :("),
    }
  }

  // if let
  let a_number: Option<u8> = Some(7);
  match a_number {
    Some(7) => println!("That's my lucky number!"),
    _ => {},
  }

  // 위 코드를 if let을 활용해서 간결하게 만들 수 있음
  if let Some(number) = a_number {
    println!("{} is my lucky number!", number);
  }

  // unwrap 메서드를 사용해서 Option 타입의 내부 값에 직접 엑세스 가능
  // 하지만 None이면 패닉 상태가 됨
  let gift = Some("candy");
  assert_eq!(gift.unwrap(), "candy");

  let empty_gift: Option<&str> = None;
  // assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!

  // expect는 unwrap과 동일하지만 메시지를 제공함
  let a = Some("value");
  assert_eq!(a.expect("fruits are healthy"), "value");

  // let b: Option<&str> = None;
  // b.expect("fruits are healthy");

  // 패닉 상태를 발생시키지 않으려면 None이면 기본값을 반환하게 한다.
  assert_eq!(Some("dog").unwrap_or("cat"), "dog");
  assert_eq!(None.unwrap_or("cat"), "cat");
}
