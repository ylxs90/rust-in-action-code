fn main() {
  let mut list: Vec<i32> = vec![1, 2, 3];
  list.push(4);

  println!("--------------------------> {}", list[1]);

  for i in list {
    if i % 2 == 0 {
      println!("{} is odd", i);
    }
  }
  
  let border: i32 = 44;
  if border > 15 {
    println!("{} > 15", border);
  }
}
