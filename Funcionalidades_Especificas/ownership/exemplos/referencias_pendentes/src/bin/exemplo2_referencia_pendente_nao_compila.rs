// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_pendentes%22%2C%20exemplo%202%2F2%20%E2%80%94%20%22Dangling%0A%2F%2F%20References%22%2C%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20%2F%2F%20A%20fun%C3%A7%C3%A3o%20abaixo%20N%C3%83O%20compila%20--%20%C3%A9%20s%C3%B3%20ilustrativa.%20Ela%20tenta%20devolver%0A%20%20%20%20%2F%2F%20uma%20refer%C3%AAncia%20para%20uma%20%60String%60%20local%2C%20que%20%C3%A9%20descartada%20%28%60drop%60%29%0A%20%20%20%20%2F%2F%20ao%20final%20da%20fun%C3%A7%C3%A3o%20--%20a%20refer%C3%AAncia%20ficaria%20apontando%20para%20mem%C3%B3ria%0A%20%20%20%20%2F%2F%20j%C3%A1%20liberada.%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20%20%20%20%20fn%20pendente%28%29%20-%3E%20%26String%20%7B%0A%20%20%20%20%2F%2F%20%20%20%20%20%20%20%20%20let%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20%2F%2F%20%20%20%20%20%20%20%20%20%26s%0A%20%20%20%20%2F%2F%20%20%20%20%20%7D%20%2F%2F%20s%20sai%20de%20escopo%20e%20%C3%A9%20descartada%20aqui%3B%20sua%20mem%C3%B3ria%20%C3%A9%0A%20%20%20%20%2F%2F%20%20%20%20%20%20%20%2F%2F%20liberada%20--%20mas%20a%20refer%C3%AAncia%20devolvida%20ainda%20apontaria%0A%20%20%20%20%2F%2F%20%20%20%20%20%20%20%2F%2F%20para%20ela%21%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20erro%5BE0106%5D%3A%20missing%20lifetime%20specifier%0A%20%20%20%20%2F%2F%20this%20function%27s%20return%20type%20contains%20a%20borrowed%20value%2C%20but%20there%0A%20%20%20%20%2F%2F%20is%20no%20value%20for%20it%20to%20be%20borrowed%20from%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20O%20compilador%20do%20Rust%20GARANTE%20que%20isso%20nunca%20aconte%C3%A7a%3A%20nenhuma%0A%20%20%20%20%2F%2F%20refer%C3%AAncia%20pode%20%22sobreviver%22%20ao%20dado%20que%20ela%20aponta.%0A%20%20%20%20println%21%28%22%28exemplo%20ilustrativo%20--%20veja%20os%20coment%C3%A1rios%20no%20c%C3%B3digo-fonte%29%22%29%3B%0A%7D%0A
//
// Notebook "referencias_pendentes", exemplo 2/2 — "Dangling
// References", cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

fn main() {
    // A função abaixo NÃO compila -- é só ilustrativa. Ela tenta devolver
    // uma referência para uma `String` local, que é descartada (`drop`)
    // ao final da função -- a referência ficaria apontando para memória
    // já liberada.
    //
    //     fn pendente() -> &String {
    //         let s = String::from("hello");
    //         &s
    //     } // s sai de escopo e é descartada aqui; sua memória é
    //       // liberada -- mas a referência devolvida ainda apontaria
    //       // para ela!
    //
    // erro[E0106]: missing lifetime specifier
    // this function's return type contains a borrowed value, but there
    // is no value for it to be borrowed from
    //
    // O compilador do Rust GARANTE que isso nunca aconteça: nenhuma
    // referência pode "sobreviver" ao dado que ela aponta.
    println!("(exemplo ilustrativo -- veja os comentários no código-fonte)");
}
