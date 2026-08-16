// Este arquivo esta incorreto de proposito.
// As duas closures tentam manter acesso mutavel simultaneo ao contador.

use std::thread;

fn main() {
    let mut contador = 0;

    thread::scope(|escopo| {
        escopo.spawn(|| {
            for _ in 0..1_000_000 {
                contador += 1;
            }
        });

        escopo.spawn(|| {
            for _ in 0..1_000_000 {
                contador += 1;
            }
        });
    });

    println!("Contador: {contador}");
}
