// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22ownership_em_funcoes%22%2C%20exemplo%201%2F2%20%E2%80%94%20%22Ownership%20and%0A%2F%2F%20Functions%22%20%28Listing%204-3%29%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%20%2F%2F%20s%20entra%20em%20escopo%0A%0A%20%20%20%20toma_posse%28s%29%3B%20%2F%2F%20o%20valor%20de%20s%20se%20move%20para%20dentro%20da%20fun%C3%A7%C3%A3o...%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20...e%20por%20isso%20n%C3%A3o%20%C3%A9%20mais%20v%C3%A1lido%20aqui%0A%0A%20%20%20%20%2F%2F%20println%21%28%22%7Bs%7D%22%29%3B%20%2F%2F%20n%C3%A3o%20compilaria%3A%20valor%20de%20s%20j%C3%A1%20foi%20movido%0A%7D%0A%0Afn%20toma_posse%28alguma_string%3A%20String%29%20%7B%20%2F%2F%20alguma_string%20entra%20em%20escopo%0A%20%20%20%20println%21%28%22%7Balguma_string%7D%22%29%3B%0A%7D%20%2F%2F%20aqui%20alguma_string%20sai%20de%20escopo%20e%20%60drop%60%20%C3%A9%20chamado%3B%20a%20mem%C3%B3ria%20%C3%A9%20liberada%0A
//
// Notebook "ownership_em_funcoes", exemplo 1/2 — "Ownership and
// Functions" (Listing 4-3), cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    let s = String::from("hello"); // s entra em escopo

    toma_posse(s); // o valor de s se move para dentro da função...
                   // ...e por isso não é mais válido aqui

    // println!("{s}"); // não compilaria: valor de s já foi movido
}

fn toma_posse(alguma_string: String) { // alguma_string entra em escopo
    println!("{alguma_string}");
} // aqui alguma_string sai de escopo e `drop` é chamado; a memória é liberada
