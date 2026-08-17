// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22escopo_e_drop%22%2C%20exemplo%201%2F3%20%E2%80%94%20%22Variable%20Scope%22%20%28Listing%0A%2F%2F%204-1%29%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20Uma%20vari%C3%A1vel%20s%C3%B3%20%C3%A9%20v%C3%A1lida%20dentro%20do%20escopo%20%28bloco%29%20em%20que%20foi%0A%2F%2F%20declarada%20%E2%80%94%20igual%20em%20Rust%20e%20em%20C.%0Afn%20main%28%29%20%7B%0A%20%20%20%20%7B%0A%20%20%20%20%20%20%20%20let%20s%20%3D%20%22hello%22%3B%20%2F%2F%20s%20%C3%A9%20v%C3%A1lida%20a%20partir%20daqui%0A%20%20%20%20%20%20%20%20println%21%28%22dentro%20do%20escopo%3A%20%7Bs%7D%22%29%3B%0A%20%20%20%20%7D%20%2F%2F%20o%20escopo%20termina%20aqui%3B%20s%20deixa%20de%20existir%0A%7D%0A
//
// Notebook "escopo_e_drop", exemplo 1/3 — "Variable Scope" (Listing
// 4-1), cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// Uma variável só é válida dentro do escopo (bloco) em que foi
// declarada — igual em Rust e em C.
fn main() {
    {
        let s = "hello"; // s é válida a partir daqui
        println!("dentro do escopo: {s}");
    } // o escopo termina aqui; s deixa de existir
}
