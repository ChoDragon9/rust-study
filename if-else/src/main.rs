fn main() {
  // [T; size]
  // T: 배열의 모든 요소에 대한 데이터 형식
  // size: 배열 길이 음수가 아닌 정수
  // - 모든 요소는 타입이 동일함
  // - 배열의 크기는 고정됨. 값만 변경할 수 있음.
  let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
  let bytes = [0; 5];

  let first = days[0];
  let second = days[1];

  // 벡터
  // - 모든 요소의 타입은 동일함
  // - 크기나 길이를 변경할 수 있음
  // - 인덱스로 접근할 수 있음
  let three_nums = vec![15, 3, 46];
  println!("Initial vector: {:?}", three_nums);

  let zeros = vec![0; 5];
  println!("Zeros: {:?}", zeros);

  let mut fruit = Vec::new();
  fruit.push("Apple");
  fruit.push("Banana");
  fruit.push("Cherry");
  println!("Fruits: {:?}", fruit);

  println!("Pop off: {:?}", fruit.pop());
  println!("Fruits: {:?}", fruit);

  let mut index_vec = vec![15, 3, 46];
  let three = index_vec[1];
  println!("Vector: {:?}, three = {}", index_vec, three);

  index_vec[1] += 5;
  println!("Vector : {:?}", index_vec);
}
