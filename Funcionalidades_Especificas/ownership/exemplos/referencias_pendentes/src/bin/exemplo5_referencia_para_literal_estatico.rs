// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_pendentes%22%2C%20exemplo%205%2F5%20%E2%80%94%20%22Dangling%0A%2F%2F%20References%22%2C%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20Outro%20caso%20em%20que%20devolver%20uma%20refer%C3%AAncia%20compila%3A%20um%20literal%20de%20string%0A%2F%2F%20%C3%A9%20%60%26%27static%20str%60%2C%20embutido%20no%20bin%C3%A1rio%20e%20vivo%20do%20in%C3%ADcio%20ao%20fim%20do%0A%2F%2F%20programa.%20N%C3%A3o%20h%C3%A1%20dono%20para%20ser%20descartado%2C%20ent%C3%A3o%20n%C3%A3o%20h%C3%A1%20como%20ficar%0A%2F%2F%20pendente%20%28compare%20com%20o%20literal%20do%20notebook%20%60strings%60%2C%20exemplo%201%29.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20r%20%3D%20saudacao%28%29%3B%0A%20%20%20%20println%21%28%22%7Br%7D%22%29%3B%0A%7D%0A%0Afn%20saudacao%28%29%20-%3E%20%26%27static%20str%20%7B%0A%20%20%20%20%22hello%22%20%2F%2F%20n%C3%A3o%20%C3%A9%20alocado%20no%20heap%20e%20n%C3%A3o%20sai%20de%20escopo%0A%7D%0A
//
// Notebook "referencias_pendentes", exemplo 5/5 — "Dangling
// References", cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// Outro caso em que devolver uma referência compila: um literal de string
// é `&'static str`, embutido no binário e vivo do início ao fim do
// programa. Não há dono para ser descartado, então não há como ficar
// pendente (compare com o literal do notebook `strings`, exemplo 1).
fn main() {
    let r = saudacao();
    println!("{r}");
}

fn saudacao() -> &'static str {
    "hello" // não é alocado no heap e não sai de escopo
}
