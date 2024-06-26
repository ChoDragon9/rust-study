mod authentication;
use regex::Regex;

fn main() {
  let mut user = authentication::User::new("jeremy", "super-secret");
  println!("The username is {}", user.get_username());
  user.set_password("even-more-secret");

  let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
  println!("Did our date match? {}", re.is_match("2024-01-01"));
}