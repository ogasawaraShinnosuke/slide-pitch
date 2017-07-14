fn main() {
  variable_test(100, 5, 0);
  let f: fn(i32, i32) -> i32 = plus_number_100;
  println!("4+5+100={}", f(4, 5));
  loop_test(5);
}

fn plus_number_100(x: i32, y: i32) -> i32 { x + y + 100 }

fn variable_test(x: i32, y: i32, z: i32) {
  if x >= 100 {
    println!("invalid number.");
  }
  println!("z={}", z);
  {
    let mut z = 0;
    println!("z={}", z);
    z = 10;
    println!("z={}", z);
  }
  println!("x={}, y={}, z={}", x, y, z);
}

fn loop_test(mut x: i32) {
  let mut done = false;
  while !done {
    x += x - 3;
    println!("{}", x);
    if x % 5 == 0 {
      done = true;
    }
  }
}
