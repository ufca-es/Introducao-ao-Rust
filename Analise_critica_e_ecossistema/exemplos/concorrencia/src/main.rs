use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let contador = Arc::new(Mutex::new(0));
    let mut tarefas = Vec::new();

    for _ in 0..5 {
        let contador_compartilhado = Arc::clone(&contador);
        tarefas.push(thread::spawn(move || {
            let mut valor = contador_compartilhado.lock().expect("mutex bloqueado");
            *valor += 1;
        }));
    }

    for tarefa in tarefas {
        tarefa.join().expect("thread finalizada com erro");
    }

    println!("Valor final: {}", *contador.lock().expect("mutex bloqueado"));
}
