fn greeting(name: &str) {
  println!("Hello, {}!", name);
}

fn sum(a: i32, b: i32) -> i32 {
  return a + b;
}

fn main() {
  greeting("John");
  println!("Sum: {}", sum(3, 4));
}