// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_pendentes%22%2C%20exemplo%201%2F2%20%E2%80%94%20%22Dangling%0A%2F%2F%20References%22%2C%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20A%20solu%C3%A7%C3%A3o%20do%20Rust%20Book%20para%20%22n%C3%A3o%20pendurar%22%20uma%20refer%C3%AAncia%3A%20em%20vez%20de%0A%2F%2F%20devolver%20%60%26String%60%2C%20a%20fun%C3%A7%C3%A3o%20devolve%20a%20%60String%60%20inteira%20--%20a%20posse%20%C3%A9%0A%2F%2F%20movida%20para%20fora%2C%20n%C3%A3o%20emprestada.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s%20%3D%20sem_pendencia%28%29%3B%0A%20%20%20%20println%21%28%22%7Bs%7D%22%29%3B%0A%7D%0A%0Afn%20sem_pendencia%28%29%20-%3E%20String%20%7B%0A%20%20%20%20let%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20s%20%2F%2F%20a%20posse%20%C3%A9%20movida%20para%20fora%20da%20fun%C3%A7%C3%A3o%20--%20n%C3%A3o%20%C3%A9%20uma%20refer%C3%AAncia%2C%0A%20%20%20%20%20%20%2F%2F%20ent%C3%A3o%20nada%20fica%20pendente%0A%7D%0A
//
// Notebook "referencias_pendentes", exemplo 1/2 — "Dangling
// References", cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// A solução do Rust Book para "não pendurar" uma referência: em vez de
// devolver `&String`, a função devolve a `String` inteira -- a posse é
// movida para fora, não emprestada.
fn main() {
    let s = sem_pendencia();
    println!("{s}");
}

fn sem_pendencia() -> String {
    let s = String::from("hello");
    s // a posse é movida para fora da função -- não é uma referência,
      // então nada fica pendente
}
