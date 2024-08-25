use rand::prelude::SliceRandom;

fn main() {
    // 1에서 75까지의 숫자가 들어간 리스트를 만든다.
    const LEN: usize = 75;
    let mut list = [0; LEN];
    for i in 0..LEN {
        list[i] = i + 1;
    }

    // println!("{:?}", list);

    // 리스트의 항목을 섞는다.
    let mut rng = rand::thread_rng();
    list.shuffle(&mut rng);

    // println!("{:?}", list);

    // 24개의 요소를 꺼내 빙고 카드에 할당한 뒤 출력한다.
    let mut bingocard = [0; 24];

    for i in 0..24 {
        bingocard[i] = list[i];
    }

    println!("{:?}", bingocard);

    let mut nums = vec![1, 2, 3];
    nums.push(4);
    nums.push(5);
    println!("{:?}", nums);

    let s_vec: Vec<&str> = vec!["개", "고양이", "닭"];
    for i in s_vec {
        println!("{}", i);
    }
}
