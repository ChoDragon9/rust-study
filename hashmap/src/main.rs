use std::collections::HashMap;

fn main() {
  let mut reviews: HashMap<String, String> = HashMap::new();

  reviews.insert(String::from("Roman History"), String::from("Very"));
  reviews.insert(String::from("Cooking"), String::from("Sweet"));

  let book: &str = "Cooking";
  println!("Review for {}: {:?}", book, reviews.get(book));

  let roman: &str = "Romain History";
  println!("{} removed", roman);
  reviews.remove(roman);

  println!("Review for {}: {:?}", roman, reviews.get(roman))
}

