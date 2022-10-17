use std::convert::TryInto;

fn main() {
    let a:i32 = 10;
    let b:u16 = 100;
    let c:i32 = 400;
    let d:isize  = 1111;

    if a < b as i32 {
      println!("a < b");
    }

    let b_:i32 = b.try_into()
                .unwrap();

    println!("{}(i32) = {}(i8)", c, c as i8);
    println!("{}", d);
    if a < b_ {
      
    }

}
