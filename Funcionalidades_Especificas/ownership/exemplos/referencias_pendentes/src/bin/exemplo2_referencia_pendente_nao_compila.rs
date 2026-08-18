// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_pendentes%22%2C%20exemplo%202%2F5%20%E2%80%94%20%22Dangling%0A%2F%2F%20References%22%2C%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20%2F%2F%20A%20fun%C3%A7%C3%A3o%20abaixo%20N%C3%83O%20compila.%20Ela%20tenta%20devolver%20uma%20refer%C3%AAncia%20para%0A%20%20%20%20%2F%2F%20uma%20%60String%60%20local%2C%20que%20%C3%A9%20descartada%20ao%20final%20da%20fun%C3%A7%C3%A3o%20%E2%80%94%20a%0A%20%20%20%20%2F%2F%20refer%C3%AAncia%20apontaria%20para%20mem%C3%B3ria%20j%C3%A1%20liberada.%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20%20%20%20%20fn%20pendente%28%29%20-%3E%20%26String%20%7B%0A%20%20%20%20%2F%2F%20%20%20%20%20%20%20%20%20let%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20%2F%2F%20%20%20%20%20%20%20%20%20%26s%0A%20%20%20%20%2F%2F%20%20%20%20%20%7D%20%2F%2F%20s%20%C3%A9%20descartada%20aqui%2C%20mas%20a%20refer%C3%AAncia%20devolvida%20ainda%0A%20%20%20%20%2F%2F%20%20%20%20%20%20%20%2F%2F%20apontaria%20para%20ela%21%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20erro%5BE0106%5D%3A%20missing%20lifetime%20specifier%0A%20%20%20%20%2F%2F%20%20%20this%20function%27s%20return%20type%20contains%20a%20borrowed%20value%2C%20but%20there%0A%20%20%20%20%2F%2F%20%20%20is%20no%20value%20for%20it%20to%20be%20borrowed%20from%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20O%20compilador%20GARANTE%20que%20isso%20nunca%20aconte%C3%A7a%3A%20nenhuma%20refer%C3%AAncia%0A%20%20%20%20%2F%2F%20pode%20sobreviver%20ao%20dado%20que%20ela%20aponta.%0A%20%20%20%20println%21%28%22%28exemplo%20ilustrativo%20%E2%80%94%20veja%20os%20coment%C3%A1rios%20no%20c%C3%B3digo-fonte%29%22%29%3B%0A%7D%0A
//
// Notebook "referencias_pendentes", exemplo 2/5 — "Dangling
// References", cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

fn main() {
    // A função abaixo NÃO compila. Ela tenta devolver uma referência para
    // uma `String` local, que é descartada ao final da função — a
    // referência apontaria para memória já liberada.
    //
    //     fn pendente() -> &String {
    //         let s = String::from("hello");
    //         &s
    //     } // s é descartada aqui, mas a referência devolvida ainda
    //       // apontaria para ela!
    //
    // erro[E0106]: missing lifetime specifier
    //   this function's return type contains a borrowed value, but there
    //   is no value for it to be borrowed from
    //
    // O compilador GARANTE que isso nunca aconteça: nenhuma referência
    // pode sobreviver ao dado que ela aponta.
    println!("(exemplo ilustrativo — veja os comentários no código-fonte)");
}
