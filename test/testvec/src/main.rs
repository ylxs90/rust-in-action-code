
fn main() {
  let mut list: Vec<i32> = vec![1, 2, 3];
  list.push(4);
  for i in &list {
    println!("{}", i);
  }

  println!("--------------------------> {}", list[1]);
  let  border: i32 = 44;
  if border > 15 {
    println!("{} > 15", border);
  }
}
