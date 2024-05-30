#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
  if divisor == 0.0 {
    Err(DivisionByZeroError)
  } else {
    Ok(dividend / divisor)
  }
}

fn main() {
  // Result<T, E>: 오류를 반환하고 전파하는 열거형을 제공함
  // enum Result<T, E> {
  //   Ok(T), // 성공을 나타내고 값을 포함
  //   Err(E) // 오류를 나타내고 오류 값을 포함
  // }

  println!("{:?}", safe_division(9.0, 3.0));
  println!("{:?}", safe_division(4.0, 0.0));
  println!("{:?}", safe_division(0.0, 2.0));
}
