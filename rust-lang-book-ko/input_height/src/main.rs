fn main() {
    let mut height;

    loop {
        println!("키(cm) :");
        // height를 입력 받음
        height = input_f(0.0);
        // height가 0.0 보다 크면 멈춤
        if height > 0.0 {
            break;
        }
        // height 유효한 값 설명
        println!("height는 0보다 커야됨!")
    }

    let standard_weight = 22.0 * (height / 100.0).powf(2.0);
    println!("표준 체중은 {:.1}kg입니다.", standard_weight);
}

fn input_f(def: f64) -> f64 {
    let s = input_s();
    match s.trim().parse() {
        Ok(s) => s,
        Err(_) => def,
    }
}

fn input_s() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("입력 에러");

    s
}
