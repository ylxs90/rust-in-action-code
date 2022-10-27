use rand::{thread_rng, Rng};
use std::io;

fn main() {
  let mut guess = String::new();
  let mut r = thread_rng();
  let n: u32 = r.gen_range(0..11);
  println!("{}", n);

  io::stdin().read_line(&mut guess).expect("error");

  println!("input {}", guess);
}
