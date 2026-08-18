// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_pendentes%22%2C%20exemplo%203%2F5%20%E2%80%94%20%22Dangling%0A%2F%2F%20References%22%2C%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20A%20mesma%20regra%20do%20exemplo%202%2C%20agora%20sem%20fun%C3%A7%C3%A3o%20nenhuma%20no%20meio%3A%20basta%0A%2F%2F%20que%20o%20dono%20morra%20antes%20da%20refer%C3%AAncia%20para%20o%20compilador%20recusar.%0Afn%20main%28%29%20%7B%0A%20%20%20%20%2F%2F%20O%20trecho%20abaixo%20N%C3%83O%20compila.%20Descomente%20para%20ver%20o%20erro%3A%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20%20%20%20%20let%20r%3B%0A%20%20%20%20%2F%2F%20%20%20%20%20%7B%0A%20%20%20%20%2F%2F%20%20%20%20%20%20%20%20%20let%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20%2F%2F%20%20%20%20%20%20%20%20%20r%20%3D%20%26s%3B%0A%20%20%20%20%2F%2F%20%20%20%20%20%7D%20%2F%2F%20s%20%C3%A9%20descartada%20aqui%0A%20%20%20%20%2F%2F%20%20%20%20%20println%21%28%22%7Br%7D%22%29%3B%20%2F%2F%20r%20apontaria%20para%20mem%C3%B3ria%20j%C3%A1%20liberada%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20erro%5BE0597%5D%3A%20%60s%60%20does%20not%20live%20long%20enough%0A%20%20%20%20println%21%28%22%28exemplo%20ilustrativo%20%E2%80%94%20veja%20os%20coment%C3%A1rios%20no%20c%C3%B3digo-fonte%29%22%29%3B%0A%7D%0A
//
// Notebook "referencias_pendentes", exemplo 3/5 — "Dangling
// References", cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// A mesma regra do exemplo 2, agora sem função nenhuma no meio: basta
// que o dono morra antes da referência para o compilador recusar.
fn main() {
    // O trecho abaixo NÃO compila. Descomente para ver o erro:
    //
    //     let r;
    //     {
    //         let s = String::from("hello");
    //         r = &s;
    //     } // s é descartada aqui
    //     println!("{r}"); // r apontaria para memória já liberada
    //
    // erro[E0597]: `s` does not live long enough
    println!("(exemplo ilustrativo — veja os comentários no código-fonte)");
}
