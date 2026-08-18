// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22retorno_e_escopo%22%2C%20exemplo%201%2F5%20%E2%80%94%20%22Return%20Values%20and%20Scope%22%0A%2F%2F%20%28Listing%204-4%29%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s1%20%3D%20da_posse%28%29%3B%20%2F%2F%20o%20valor%20de%20retorno%20se%20move%20para%20s1%0A%20%20%20%20println%21%28%22s1%20%3D%20%7Bs1%7D%22%29%3B%0A%7D%0A%0Afn%20da_posse%28%29%20-%3E%20String%20%7B%0A%20%20%20%20let%20alguma_string%20%3D%20String%3A%3Afrom%28%22yours%22%29%3B%0A%20%20%20%20alguma_string%20%2F%2F%20devolvida%3A%20a%20posse%20se%20move%20para%20quem%20chamou%0A%7D%0A
//
// Notebook "retorno_e_escopo", exemplo 1/5 — "Return Values and Scope"
// (Listing 4-4), cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    let s1 = da_posse(); // o valor de retorno se move para s1
    println!("s1 = {s1}");
}

fn da_posse() -> String {
    let alguma_string = String::from("yours");
    alguma_string // devolvida: a posse se move para quem chamou
}
