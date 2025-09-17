use std::thread;
use std::time::Duration;

fn main() {
  let hanlderOne = thread::spawn(|| {
    for i in 1..5 {
      println!("Thread one : {}", i);
      thread::sleep(Duration::from_millis(500));
    }
  });

  let handleTwo = thread::spawn(|| {
    for i in 1..=5 {
        println!("Thread two: {}", i);
        thread::sleep(Duration::from_millis(300));
    }
  });

    hanlderOne.join().unwrap();
    handleTwo.join().unwrap();

    println!("Fim!");
}