fn main() {
    for i500 in 0..11 {
        for i100 in 0..4 {
            for i50 in 0..11 {
                let total = i500 * 500 + i100 * 100 + i50 * 50;
                if total == 3950 {
                    println!("500원x{}+100원x{}+50원x{}=3950", i500, i100, i50);
                }
            }
        }
    }
}
