// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_pendentes%22%2C%20exemplo%204%2F5%20%E2%80%94%20%22Dangling%0A%2F%2F%20References%22%2C%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20Devolver%20uma%20refer%C3%AAncia%20n%C3%A3o%20%C3%A9%20proibido%20%E2%80%94%20proibido%20%C3%A9%20devolver%20uma%0A%2F%2F%20refer%C3%AAncia%20para%20um%20dado%20que%20morre%20junto%20com%20a%20fun%C3%A7%C3%A3o.%20Aqui%20a%0A%2F%2F%20refer%C3%AAncia%20devolvida%20vem%20do%20PAR%C3%82METRO%3A%20o%20dono%20%C3%A9%20quem%20chamou%2C%20e%20o%20dado%0A%2F%2F%20continua%20vivo%20depois%20do%20retorno.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%0A%20%20%20%20let%20r%20%3D%20a_mesma%28%26s%29%3B%0A%0A%20%20%20%20println%21%28%22%7Br%7D%22%29%3B%0A%7D%20%2F%2F%20drop%28s%29%20s%C3%B3%20aqui%2C%20no%20dono%0A%0Afn%20a_mesma%28s%3A%20%26String%29%20-%3E%20%26String%20%7B%0A%20%20%20%20s%20%2F%2F%20o%20dado%20%C3%A9%20de%20quem%20chamou%3B%20nada%20%C3%A9%20descartado%20no%20fim%20da%20fun%C3%A7%C3%A3o%0A%7D%0A
//
// Notebook "referencias_pendentes", exemplo 4/5 — "Dangling
// References", cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// Devolver uma referência não é proibido — proibido é devolver uma
// referência para um dado que morre junto com a função. Aqui a
// referência devolvida vem do PARÂMETRO: o dono é quem chamou, e o dado
// continua vivo depois do retorno.
fn main() {
    let s = String::from("hello");

    let r = a_mesma(&s);

    println!("{r}");
} // drop(s) só aqui, no dono

fn a_mesma(s: &String) -> &String {
    s // o dado é de quem chamou; nada é descartado no fim da função
}
