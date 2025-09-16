fn main() {
  let name = String::from("Rust"); // name é o dono
  let other = name; // ownership move par other

  // println!("{}", nome); ERRO! `nome` não é mais válido

  println!("{}", other);
  // Strings são tipos que alocam memória na heap, então o ownership é movido, não copiado
}