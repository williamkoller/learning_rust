use std::thread;
use std::time::Duration;

fn main() {
  let handle_one = thread::spawn(|| {
    for i in 1..5 {
      println!("Thread one : {}", i);
      thread::sleep(Duration::from_millis(500));
    }
  });

  let handle_two = thread::spawn(|| {
    for i in 1..= 5 {
      println!("Thread two: {}", i);
      thread::sleep(Duration::from_millis(300));
    }
  });

  handle_one.join().unwrap();
  handle_two.join().unwrap();

  println!("Fim!");
}