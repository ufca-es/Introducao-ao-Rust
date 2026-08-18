// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_mutaveis%22%2C%20exemplo%204%2F5%20%E2%80%94%20%22Mutable%20References%22%2C%0A%2F%2F%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20O%20escopo%20de%20uma%20refer%C3%AAncia%20vai%20da%20cria%C3%A7%C3%A3o%20at%C3%A9%20o%20%C3%9ALTIMO%20USO%20dela%20%28NLL%20%E2%80%94%0A%2F%2F%20non-lexical%20lifetimes%29%2C%20n%C3%A3o%20at%C3%A9%20o%20fim%20literal%20do%20bloco.%20%C3%89%20por%20isso%20que%0A%2F%2F%20o%20c%C3%B3digo%20abaixo%20compila%2C%20apesar%20de%20parecer%20violar%20a%20regra%202.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%0A%20%20%20%20let%20r1%20%3D%20%26s%3B%0A%20%20%20%20let%20r2%20%3D%20%26s%3B%0A%20%20%20%20println%21%28%22%7Br1%7D%20e%20%7Br2%7D%22%29%3B%20%2F%2F%20r1%20e%20r2%20n%C3%A3o%20s%C3%A3o%20mais%20usadas%20depois%20daqui%0A%0A%20%20%20%20let%20r3%20%3D%20%26mut%20s%3B%20%2F%2F%20ok%3A%20os%20empr%C3%A9stimos%20imut%C3%A1veis%20j%C3%A1%20terminaram%0A%20%20%20%20r3.push_str%28%22%2C%20world%22%29%3B%0A%20%20%20%20println%21%28%22%7Br3%7D%22%29%3B%0A%7D%0A
//
// Notebook "referencias_mutaveis", exemplo 4/5 — "Mutable References",
// cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// O escopo de uma referência vai da criação até o ÚLTIMO USO dela (NLL —
// non-lexical lifetimes), não até o fim literal do bloco. É por isso que
// o código abaixo compila, apesar de parecer violar a regra 2.
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} e {r2}"); // r1 e r2 não são mais usadas depois daqui

    let r3 = &mut s; // ok: os empréstimos imutáveis já terminaram
    r3.push_str(", world");
    println!("{r3}");
}
