use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let task1 = tokio::spawn(async {
        for i in 1..=3 {
            println!("Tarefa 1: {}", i);
            sleep(Duration::from_millis(500)).await;
        }
    });

    let task2 = tokio::spawn(async {
        for i in 1..=3 {
            println!("Tarefa 2: {}", i);
            sleep(Duration::from_millis(300)).await;
        }
    });

    task1.await.unwrap();
    task2.await.unwrap();

    println!("Tudo pronto!");
}
