fn main() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[test]
fn add_works() {
  assert_eq!(add(1, 2), 3);
  assert_eq!(add(10, 12), 22);
  assert_eq!(add(5, -2), 3);
}

#[test]
#[should_panic] // 예상된 실패는 should_panic을 사용함
fn add_fails() {
  assert_eq!(add(2, 2), 7);
}

#[test]
#[ignore = "not yet reviewed by the Q.A. team"]
fn add_negatives() {
  assert_eq!(add(-2, -2), -4);
}

#[cfg(test)]
mod add_function_tests {
  use super::*;

  #[test]
  fn add_works() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(10, 12), 22);
  }

  #[test]
  #[should_panic]
  fn add_fails() {
    assert_eq!(add(2, 7), 7);
  }

  #[test]
  #[ignore]
  fn add_negatives() {
    assert_eq!(add(-2, -2), -4);
  }
}

pub fn is_even(num: i32) -> bool {
  num % 2 == 0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_true_when_even() {
    assert_eq!(is_even(2), true);
  }

  #[test]
  fn is_false_when_odd() {
    assert_eq!(is_even(1), false);
  }
}