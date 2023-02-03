fn main() {
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862]; // <1>

    for reference in haystack.iter() { // <2>
        let item = *reference; // <3>
        if item == needle {
            println!("{}", item);
        }

        // if reference == &needle { // <4>
        //   println!("{}", reference);
        // }
    }

    for item in &haystack {
      if *item == needle{
        println!("{}", item);
      }

    }
}
