// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22ownership_em_funcoes%22%2C%20exemplo%204%2F5%20%E2%80%94%20%22Ownership%20and%0A%2F%2F%20Functions%22%20%28Listing%204-3%29%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20Uma%20chamada%20que%20move%20a%20posse%20s%C3%B3%20pode%20acontecer%20UMA%20vez%20%E2%80%94%20e%20o%0A%2F%2F%20compilador%20enxerga%20isso%20mesmo%20atrav%C3%A9s%20de%20um%20la%C3%A7o.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%0A%20%20%20%20%2F%2F%20O%20la%C3%A7o%20abaixo%20N%C3%83O%20compila.%20Descomente%20para%20ver%20o%20erro%3A%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20%20%20%20%20for%20_%20in%200..3%20%7B%0A%20%20%20%20%2F%2F%20%20%20%20%20%20%20%20%20toma_posse%28s%29%3B%0A%20%20%20%20%2F%2F%20%20%20%20%20%7D%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20erro%5BE0382%5D%3A%20use%20of%20moved%20value%3A%20%60s%60%0A%20%20%20%20%2F%2F%20%20%20value%20moved%20here%2C%20in%20previous%20iteration%20of%20loop%0A%0A%20%20%20%20toma_posse%28s%29%3B%20%2F%2F%20uma%20%C3%BAnica%20chamada%20%C3%A9%20o%20que%20a%20posse%20permite%0A%7D%0A%0Afn%20toma_posse%28alguma_string%3A%20String%29%20%7B%0A%20%20%20%20println%21%28%22%7Balguma_string%7D%22%29%3B%0A%7D%0A
//
// Notebook "ownership_em_funcoes", exemplo 4/5 — "Ownership and
// Functions" (Listing 4-3), cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// Uma chamada que move a posse só pode acontecer UMA vez — e o
// compilador enxerga isso mesmo através de um laço.
fn main() {
    let s = String::from("hello");

    // O laço abaixo NÃO compila. Descomente para ver o erro:
    //
    //     for _ in 0..3 {
    //         toma_posse(s);
    //     }
    //
    // erro[E0382]: use of moved value: `s`
    //   value moved here, in previous iteration of loop

    toma_posse(s); // uma única chamada é o que a posse permite
}

fn toma_posse(alguma_string: String) {
    println!("{alguma_string}");
}
