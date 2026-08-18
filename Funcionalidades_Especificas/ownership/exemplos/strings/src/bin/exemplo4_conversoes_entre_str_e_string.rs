// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22strings%22%2C%20exemplo%204%2F5%20%E2%80%94%20%22The%20String%20Type%22%2C%20cap.%204.1%20do%20Rust%0A%2F%2F%20Book%3A%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20As%20duas%20dire%C3%A7%C3%B5es%20da%20convers%C3%A3o%20t%C3%AAm%20custos%20bem%20diferentes%3A%20virar%0A%2F%2F%20%60String%60%20ALOCA%20no%20heap%20e%20copia%20os%20bytes%3B%20virar%20%60%26str%60%20%C3%A9%20s%C3%B3%20emprestar%20o%0A%2F%2F%20buffer%20que%20a%20%60String%60%20j%C3%A1%20tem%2C%20sem%20aloca%C3%A7%C3%A3o%20nenhuma.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20literal%20%3D%20%22hello%22%3B%0A%0A%20%20%20%20let%20dinamica%20%3D%20literal.to_string%28%29%3B%20%2F%2F%20aloca%20%28o%20mesmo%20que%20String%3A%3Afrom%29%0A%20%20%20%20println%21%28%22%26str%20-%3E%20String%3A%20%7Bdinamica%7D%22%29%3B%0A%0A%20%20%20%20let%20emprestada%3A%20%26str%20%3D%20%26dinamica%3B%20%2F%2F%20s%C3%B3%20empresta%20o%20buffer%20existente%0A%20%20%20%20println%21%28%22String%20-%3E%20%26str%3A%20%7Bemprestada%7D%22%29%3B%0A%0A%20%20%20%20println%21%28%22mesmo%20conte%C3%BAdo%3F%20%7B%7D%22%2C%20literal%20%3D%3D%20emprestada%29%3B%0A%7D%0A
//
// Notebook "strings", exemplo 4/5 — "The String Type", cap. 4.1 do Rust
// Book: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// As duas direções da conversão têm custos bem diferentes: virar
// `String` ALOCA no heap e copia os bytes; virar `&str` é só emprestar o
// buffer que a `String` já tem, sem alocação nenhuma.
fn main() {
    let literal = "hello";

    let dinamica = literal.to_string(); // aloca (o mesmo que String::from)
    println!("&str -> String: {dinamica}");

    let emprestada: &str = &dinamica; // só empresta o buffer existente
    println!("String -> &str: {emprestada}");

    println!("mesmo conteúdo? {}", literal == emprestada);
}
