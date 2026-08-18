// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22retorno_e_escopo%22%2C%20exemplo%202%2F5%20%E2%80%94%20%22Return%20Values%20and%20Scope%22%0A%2F%2F%20%28Listing%204-4%29%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s2%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20let%20s3%20%3D%20pega_e_devolve%28s2%29%3B%20%2F%2F%20s2%20se%20move%20para%20a%20fun%C3%A7%C3%A3o%2C%20que%20move%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20seu%20retorno%20para%20s3%0A%20%20%20%20println%21%28%22s3%20%3D%20%7Bs3%7D%22%29%3B%0A%7D%0A%0Afn%20pega_e_devolve%28uma_string%3A%20String%29%20-%3E%20String%20%7B%0A%20%20%20%20uma_string%0A%7D%0A
//
// Notebook "retorno_e_escopo", exemplo 2/5 — "Return Values and Scope"
// (Listing 4-4), cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    let s2 = String::from("hello");
    let s3 = pega_e_devolve(s2); // s2 se move para a função, que move
                                 // seu retorno para s3
    println!("s3 = {s3}");
}

fn pega_e_devolve(uma_string: String) -> String {
    uma_string
}
