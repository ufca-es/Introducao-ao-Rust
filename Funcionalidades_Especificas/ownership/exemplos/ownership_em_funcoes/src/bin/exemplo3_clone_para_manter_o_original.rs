// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22ownership_em_funcoes%22%2C%20exemplo%203%2F5%20%E2%80%94%20%22Ownership%20and%0A%2F%2F%20Functions%22%20%28Listing%204-3%29%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20Primeira%20sa%C3%ADda%20para%20o%20impasse%20do%20exemplo%201%3A%20%60clone%28%29%60%20duplica%20o%20buffer%0A%2F%2F%20do%20heap%2C%20ent%C3%A3o%20a%20fun%C3%A7%C3%A3o%20recebe%20a%20posse%20da%20C%C3%93PIA%20e%20o%20original%20continua%0A%2F%2F%20v%C3%A1lido.%20O%20pre%C3%A7o%20%C3%A9%20duplicar%20o%20heap%20a%20cada%20chamada%20%E2%80%94%20o%20cap.%204.2%20resolve%0A%2F%2F%20o%20mesmo%20problema%20com%20refer%C3%AAncias%2C%20sem%20c%C3%B3pia%20nenhuma.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%0A%20%20%20%20toma_posse%28s.clone%28%29%29%3B%20%2F%2F%20a%20c%C3%B3pia%20%C3%A9%20movida%3B%20%60s%60%20n%C3%A3o%0A%0A%20%20%20%20println%21%28%22s%20continua%20v%C3%A1lida%3A%20%7Bs%7D%22%29%3B%0A%7D%0A%0Afn%20toma_posse%28alguma_string%3A%20String%29%20%7B%0A%20%20%20%20println%21%28%22%7Balguma_string%7D%22%29%3B%0A%7D%20%2F%2F%20s%C3%B3%20a%20C%C3%93PIA%20%C3%A9%20liberada%20aqui%3B%20o%20original%20em%20%60main%60%20segue%20vivo%0A
//
// Notebook "ownership_em_funcoes", exemplo 3/5 — "Ownership and
// Functions" (Listing 4-3), cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// Primeira saída para o impasse do exemplo 1: `clone()` duplica o buffer
// do heap, então a função recebe a posse da CÓPIA e o original continua
// válido. O preço é duplicar o heap a cada chamada — o cap. 4.2 resolve
// o mesmo problema com referências, sem cópia nenhuma.
fn main() {
    let s = String::from("hello");

    toma_posse(s.clone()); // a cópia é movida; `s` não

    println!("s continua válida: {s}");
}

fn toma_posse(alguma_string: String) {
    println!("{alguma_string}");
} // só a CÓPIA é liberada aqui; o original em `main` segue vivo
