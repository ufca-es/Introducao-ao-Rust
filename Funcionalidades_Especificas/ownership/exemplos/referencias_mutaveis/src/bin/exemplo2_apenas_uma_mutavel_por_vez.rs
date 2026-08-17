// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_mutaveis%22%2C%20exemplo%202%2F4%20%E2%80%94%20%22Mutable%20References%22%2C%0A%2F%2F%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20Regra%201%3A%20s%C3%B3%20pode%20existir%20UMA%20refer%C3%AAncia%20mut%C3%A1vel%20para%20um%20dado%20em%20um%0A%2F%2F%20dado%20escopo%20--%20isso%20evita%20data%20races%20em%20tempo%20de%20COMPILA%C3%87%C3%83O.%0Afn%20main%28%29%20%7B%0A%20%20%20%20%2F%2F%20O%20trecho%20abaixo%20N%C3%83O%20compila.%20Descomente%20para%20ver%20o%20erro%3A%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20%20%20%20%20let%20mut%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20%2F%2F%20%20%20%20%20let%20r1%20%3D%20%26mut%20s%3B%0A%20%20%20%20%2F%2F%20%20%20%20%20let%20r2%20%3D%20%26mut%20s%3B%0A%20%20%20%20%2F%2F%20%20%20%20%20println%21%28%22%7Br1%7D%2C%20%7Br2%7D%22%29%3B%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20erro%5BE0499%5D%3A%20cannot%20borrow%20%60s%60%20as%20mutable%20more%20than%20once%20at%20a%20time%0A%20%20%20%20println%21%28%22%28exemplo%20ilustrativo%20--%20veja%20os%20coment%C3%A1rios%20no%20c%C3%B3digo-fonte%29%22%29%3B%0A%7D%0A
//
// Notebook "referencias_mutaveis", exemplo 2/4 — "Mutable References",
// cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// Regra 1: só pode existir UMA referência mutável para um dado em um
// dado escopo -- isso evita data races em tempo de COMPILAÇÃO.
fn main() {
    // O trecho abaixo NÃO compila. Descomente para ver o erro:
    //
    //     let mut s = String::from("hello");
    //     let r1 = &mut s;
    //     let r2 = &mut s;
    //     println!("{r1}, {r2}");
    //
    // erro[E0499]: cannot borrow `s` as mutable more than once at a time
    println!("(exemplo ilustrativo -- veja os comentários no código-fonte)");
}
