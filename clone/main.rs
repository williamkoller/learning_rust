fn main() {
  let name = String::from("Rust");
  let other = name.clone(); // Agora temos duas Strings independentes

  println!("{} e {}", name, other);
}