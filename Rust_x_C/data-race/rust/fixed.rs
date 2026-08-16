use std::sync::{Arc, Mutex};
use std::thread;

const INCREMENTOS: usize = 1_000_000;

fn main() {
    let contador = Arc::new(Mutex::new(0));
    let mut threads = Vec::new();

    for _ in 0..2 {
        let contador_compartilhado = Arc::clone(&contador);

        threads.push(thread::spawn(move || {
            for _ in 0..INCREMENTOS {
                let mut valor = contador_compartilhado.lock().unwrap();
                *valor += 1;
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Contador: {}", *contador.lock().unwrap());
}
