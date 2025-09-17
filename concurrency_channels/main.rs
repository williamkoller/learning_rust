use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mensagens = vec!["Olá", "do", "thread"];
        for msg in mensagens {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recebida in rx {
        println!("Recebido: {}", recebida);
    }
}
