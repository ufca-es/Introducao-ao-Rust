// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_mutaveis%22%2C%20exemplo%203%2F4%20%E2%80%94%20%22Mutable%20References%22%2C%0A%2F%2F%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20Regra%202%3A%20tamb%C3%A9m%20n%C3%A3o%20%C3%A9%20permitido%20misturar%20uma%20refer%C3%AAncia%20mut%C3%A1vel%20com%20uma%0A%2F%2F%20imut%C3%A1vel%20enquanto%20a%20imut%C3%A1vel%20ainda%20estiver%20em%20uso.%0Afn%20main%28%29%20%7B%0A%20%20%20%20%2F%2F%20O%20trecho%20abaixo%20tamb%C3%A9m%20N%C3%83O%20compila%3A%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20%20%20%20%20let%20mut%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20%2F%2F%20%20%20%20%20let%20r1%20%3D%20%26s%3B%20%20%20%20%20%20%2F%2F%20ok%3A%20refer%C3%AAncia%20imut%C3%A1vel%0A%20%20%20%20%2F%2F%20%20%20%20%20let%20r2%20%3D%20%26s%3B%20%20%20%20%20%20%2F%2F%20ok%3A%20outra%20imut%C3%A1vel%2C%20sem%20problema%0A%20%20%20%20%2F%2F%20%20%20%20%20let%20r3%20%3D%20%26mut%20s%3B%20%20%2F%2F%20erro%5BE0502%5D%3A%20cannot%20borrow%20%60s%60%20as%20mutable%0A%20%20%20%20%2F%2F%20%20%20%20%20println%21%28%22%7Br1%7D%2C%20%7Br2%7D%2C%20%7Br3%7D%22%29%3B%20%2F%2F%20porque%20%60s%60%20tamb%C3%A9m%20est%C3%A1%0A%20%20%20%20%2F%2F%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20emprestada%20como%20imut%C3%A1vel%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20M%C3%BAltiplas%20refer%C3%AAncias%20imut%C3%A1veis%20s%C3%A3o%20permitidas%20ao%20mesmo%20tempo%0A%20%20%20%20%2F%2F%20porque%20nenhuma%20delas%20pode%20alterar%20o%20dado%20--%20leitura%20simult%C3%A2nea%20n%C3%A3o%0A%20%20%20%20%2F%2F%20%C3%A9%20um%20data%20race%20%28veja%20%60referencias_e_borrowing%60%29.%0A%20%20%20%20println%21%28%22%28exemplo%20ilustrativo%20--%20veja%20os%20coment%C3%A1rios%20no%20c%C3%B3digo-fonte%29%22%29%3B%0A%7D%0A
//
// Notebook "referencias_mutaveis", exemplo 3/4 — "Mutable References",
// cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// Regra 2: também não é permitido misturar uma referência mutável com uma
// imutável enquanto a imutável ainda estiver em uso.
fn main() {
    // O trecho abaixo também NÃO compila:
    //
    //     let mut s = String::from("hello");
    //     let r1 = &s;      // ok: referência imutável
    //     let r2 = &s;      // ok: outra imutável, sem problema
    //     let r3 = &mut s;  // erro[E0502]: cannot borrow `s` as mutable
    //     println!("{r1}, {r2}, {r3}"); // porque `s` também está
    //                                   // emprestada como imutável
    //
    // Múltiplas referências imutáveis são permitidas ao mesmo tempo
    // porque nenhuma delas pode alterar o dado -- leitura simultânea não
    // é um data race (veja `referencias_e_borrowing`).
    println!("(exemplo ilustrativo -- veja os comentários no código-fonte)");
}
