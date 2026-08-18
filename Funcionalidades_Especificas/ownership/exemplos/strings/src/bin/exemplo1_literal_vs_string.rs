// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22strings%22%2C%20exemplo%201%2F5%20%E2%80%94%20%22The%20String%20Type%22%2C%20cap.%204.1%20do%20Rust%0A%2F%2F%20Book%3A%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20%60String%60%20%C3%A9%20o%20exemplo%20can%C3%B4nico%20de%20dado%20de%20TAMANHO%20DESCONHECIDO%20em%20tempo%0A%2F%2F%20de%20compila%C3%A7%C3%A3o%20e%20EXPANS%C3%8DVEL%20em%20tempo%20de%20execu%C3%A7%C3%A3o%20%E2%80%94%20por%20isso%20vive%20no%0A%2F%2F%20heap.%20J%C3%A1%20o%20literal%20%28%60%26str%60%29%20tem%20tamanho%20conhecido%2C%20fica%20embutido%20no%0A%2F%2F%20bin%C3%A1rio%20e%20%C3%A9%20sempre%20imut%C3%A1vel.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20literal%20%3D%20%22hello%22%3B%0A%20%20%20%20println%21%28%22literal%3A%20%7Bliteral%7D%22%29%3B%0A%0A%20%20%20%20let%20dinamica%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%20%2F%2F%20alocada%20no%20heap%0A%20%20%20%20println%21%28%22dinamica%3A%20%7Bdinamica%7D%22%29%3B%0A%7D%0A
//
// Notebook "strings", exemplo 1/5 — "The String Type", cap. 4.1 do Rust
// Book: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// `String` é o exemplo canônico de dado de TAMANHO DESCONHECIDO em tempo
// de compilação e EXPANSÍVEL em tempo de execução — por isso vive no
// heap. Já o literal (`&str`) tem tamanho conhecido, fica embutido no
// binário e é sempre imutável.
fn main() {
    let literal = "hello";
    println!("literal: {literal}");

    let dinamica = String::from("hello"); // alocada no heap
    println!("dinamica: {dinamica}");
}
