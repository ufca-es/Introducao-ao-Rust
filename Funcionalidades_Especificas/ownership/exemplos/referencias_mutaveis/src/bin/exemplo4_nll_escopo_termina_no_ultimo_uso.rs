// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_mutaveis%22%2C%20exemplo%204%2F4%20%E2%80%94%20%22Mutable%20References%22%2C%0A%2F%2F%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20O%20escopo%20de%20uma%20refer%C3%AAncia%20come%C3%A7a%20onde%20ela%20%C3%A9%20criada%20e%20vai%20at%C3%A9%20a%0A%2F%2F%20%C3%9ALTIMA%20vez%20que%20%C3%A9%20usada%20%28NLL%20--%20non-lexical%20lifetimes%29%2C%20n%C3%A3o%20at%C3%A9%20o%20fim%0A%2F%2F%20literal%20do%20bloco.%20Por%20isso%20o%20c%C3%B3digo%20abaixo%20compila%3A%20r1%20e%20r2%20n%C3%A3o%20s%C3%A3o%0A%2F%2F%20mais%20usadas%20depois%20do%20println%21%2C%20ent%C3%A3o%20%60s%60%20j%C3%A1%20pode%20ser%20emprestada%20como%0A%2F%2F%20mut%C3%A1vel%20em%20seguida.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%0A%20%20%20%20let%20r1%20%3D%20%26s%3B%0A%20%20%20%20let%20r2%20%3D%20%26s%3B%0A%20%20%20%20println%21%28%22%7Br1%7D%20e%20%7Br2%7D%22%29%3B%20%2F%2F%20r1%20e%20r2%20n%C3%A3o%20s%C3%A3o%20mais%20usadas%20depois%20daqui%0A%0A%20%20%20%20let%20r3%20%3D%20%26mut%20s%3B%20%2F%2F%20ok%3A%20o%20empr%C3%A9stimo%20imut%C3%A1vel%20j%C3%A1%20%22terminou%22%0A%20%20%20%20r3.push_str%28%22%2C%20world%22%29%3B%0A%20%20%20%20println%21%28%22%7Br3%7D%22%29%3B%0A%7D%0A
//
// Notebook "referencias_mutaveis", exemplo 4/4 — "Mutable References",
// cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// O escopo de uma referência começa onde ela é criada e vai até a
// ÚLTIMA vez que é usada (NLL -- non-lexical lifetimes), não até o fim
// literal do bloco. Por isso o código abaixo compila: r1 e r2 não são
// mais usadas depois do println!, então `s` já pode ser emprestada como
// mutável em seguida.
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} e {r2}"); // r1 e r2 não são mais usadas depois daqui

    let r3 = &mut s; // ok: o empréstimo imutável já "terminou"
    r3.push_str(", world");
    println!("{r3}");
}
