// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22move%22%2C%20exemplo%201%2F5%20%E2%80%94%20%22Ways%20Variables%20and%20Data%20Interact%3A%0A%2F%2F%20Move%22%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s1%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20let%20s2%20%3D%20s1%3B%20%2F%2F%20move%3A%20a%20posse%20do%20buffer%20no%20heap%20passa%20de%20s1%20para%20s2%0A%0A%20%20%20%20println%21%28%22s2%20%3D%20%7Bs2%7D%22%29%3B%0A%0A%20%20%20%20%2F%2F%20A%20linha%20abaixo%20N%C3%83O%20compila.%20Descomente%20para%20ver%20o%20erro%3A%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20%20%20%20%20println%21%28%22s1%20%3D%20%7Bs1%7D%22%29%3B%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20erro%5BE0382%5D%3A%20borrow%20of%20moved%20value%3A%20%60s1%60%0A%20%20%20%20%2F%2F%20%20%20move%20occurs%20because%20%60s1%60%20has%20type%20%60String%60%2C%20which%20does%20not%0A%20%20%20%20%2F%2F%20%20%20implement%20the%20%60Copy%60%20trait%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20A%20atribui%C3%A7%C3%A3o%20copiou%20s%C3%B3%20ponteiro%2C%20tamanho%20e%20capacidade%20%E2%80%94%20nunca%20o%0A%20%20%20%20%2F%2F%20conte%C3%BAdo%20do%20heap.%20Se%20os%20dois%20nomes%20continuassem%20v%C3%A1lidos%2C%20ao%20sa%C3%ADrem%0A%20%20%20%20%2F%2F%20de%20escopo%20o%20Rust%20chamaria%20%60drop%60%20duas%20vezes%20sobre%20o%20MESMO%20endere%C3%A7o%3A%0A%20%20%20%20%2F%2F%20um%20double%20free.%20Em%20vez%20de%20deixar%20isso%20quebrar%20em%20tempo%20de%20execu%C3%A7%C3%A3o%2C%0A%20%20%20%20%2F%2F%20o%20compilador%20invalida%20%60s1%60%20no%20momento%20do%20move.%0A%7D%0A
//
// Notebook "move", exemplo 1/5 — "Ways Variables and Data Interact:
// Move", cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // move: a posse do buffer no heap passa de s1 para s2

    println!("s2 = {s2}");

    // A linha abaixo NÃO compila. Descomente para ver o erro:
    //
    //     println!("s1 = {s1}");
    //
    // erro[E0382]: borrow of moved value: `s1`
    //   move occurs because `s1` has type `String`, which does not
    //   implement the `Copy` trait
    //
    // A atribuição copiou só ponteiro, tamanho e capacidade — nunca o
    // conteúdo do heap. Se os dois nomes continuassem válidos, ao saírem
    // de escopo o Rust chamaria `drop` duas vezes sobre o MESMO endereço:
    // um double free. Em vez de deixar isso quebrar em tempo de execução,
    // o compilador invalida `s1` no momento do move.
}
