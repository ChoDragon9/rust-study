use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    for _ in 0..=4 {
        let dice = rng.gen_range(1..=6);
        println!("{}", dice);
    }

    let r = 3..15;
    println!("{}..{}", r.start, r.end);
}
