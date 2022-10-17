use std::thread;
use std::thread::JoinHandle;

fn main() {
  let t : JoinHandle<_> = thread::spawn(|| {
    println!("hello world");
    return "123";
  });

  println!("{:?}", t);
  let ret = t.join();
  println!("{:#?}", ret);
}

#[cfg(test)]
mod testadd {

  fn add(a: i32, b: i32) -> i32 {
    return a + b;
  }

  
  #[test]
  fn test_add() {
   
    assert_eq!(add(3i32, 15i32), 18i32);
  }
}
