// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22strings%22%2C%20exemplo%201%2F3%20%E2%80%94%20%22The%20String%20Type%22%2C%20cap.%204.1%20do%20Rust%0A%2F%2F%20Book%3A%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%2F%2F%0A%2F%2F%20String%20%C3%A9%20o%20exemplo%20can%C3%B4nico%20de%20um%20dado%20de%20TAMANHO%20DESCONHECIDO%20EM%0A%2F%2F%20TEMPO%20DE%20COMPILA%C3%87%C3%83O%20e%20EXPANS%C3%8DVEL%20em%20tempo%20de%20execu%C3%A7%C3%A3o%20--%20por%20isso%20vive%0A%2F%2F%20no%20heap%20%28veja%20%22Heap%20versus%20Stack%22%20no%20material%20do%20cap%C3%ADtulo%29.%0A%0A%2F%2F%20Literal%20de%20string%20%28%26str%29%3A%20tamanho%20conhecido%20em%20tempo%20de%20compila%C3%A7%C3%A3o%2C%0A%2F%2F%20embutido%20no%20bin%C3%A1rio%2C%20sempre%20imut%C3%A1vel.%20%60String%3A%3Afrom%60%20aloca%20no%20heap%3A%0A%2F%2F%20pode%20crescer%20e%20ser%20mutada%2C%20desde%20que%20a%20vari%C3%A1vel%20seja%20declarada%20com%0A%2F%2F%20%60mut%60%20--%20exatamente%20porque%20seu%20tamanho%20N%C3%83O%20%C3%A9%20conhecido%20de%20antem%C3%A3o.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20literal%20%3D%20%22hello%22%3B%0A%20%20%20%20println%21%28%22literal%3A%20%7Bliteral%7D%22%29%3B%0A%0A%20%20%20%20let%20dinamica%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20println%21%28%22dinamica%3A%20%7Bdinamica%7D%22%29%3B%0A%7D%0A
//
// Notebook "strings", exemplo 1/3 — "The String Type", cap. 4.1 do Rust
// Book: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
//
// String é o exemplo canônico de um dado de TAMANHO DESCONHECIDO EM
// TEMPO DE COMPILAÇÃO e EXPANSÍVEL em tempo de execução -- por isso vive
// no heap (veja "Heap versus Stack" no material do capítulo).

// Literal de string (&str): tamanho conhecido em tempo de compilação,
// embutido no binário, sempre imutável. `String::from` aloca no heap:
// pode crescer e ser mutada, desde que a variável seja declarada com
// `mut` -- exatamente porque seu tamanho NÃO é conhecido de antemão.
fn main() {
    let literal = "hello";
    println!("literal: {literal}");

    let dinamica = String::from("hello");
    println!("dinamica: {dinamica}");
}
