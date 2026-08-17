// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22retorno_e_escopo%22%2C%20exemplo%203%2F3%20%E2%80%94%20%22Return%20Values%20and%20Scope%22%0A%2F%2F%20%28Listing%204-5%29%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20Padr%C3%A3o%20para%20%22usar%20e%20devolver%22%3A%20como%20passar%20%60s%60%20por%20valor%20move%20a%20posse%0A%2F%2F%20para%20dentro%20da%20fun%C3%A7%C3%A3o%2C%20a%20%C3%BAnica%20forma%20de%20recuper%C3%A1-la%20%C3%A9%20devolv%C3%AA-la%20de%0A%2F%2F%20volta%20%E2%80%94%20aqui%2C%20dentro%20de%20uma%20tupla%20junto%20do%20resultado.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s4%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20let%20%28s5%2C%20tamanho%29%20%3D%20calcula_tamanho%28s4%29%3B%0A%20%20%20%20println%21%28%22o%20tamanho%20de%20%27%7Bs5%7D%27%20%C3%A9%20%7Btamanho%7D%22%29%3B%0A%7D%0A%0Afn%20calcula_tamanho%28s%3A%20String%29%20-%3E%20%28String%2C%20usize%29%20%7B%0A%20%20%20%20let%20tamanho%20%3D%20s.len%28%29%3B%0A%20%20%20%20%28s%2C%20tamanho%29%20%2F%2F%20devolve%20%60s%60%20de%20volta%2C%20j%C3%A1%20que%20pass%C3%A1-la%20por%20valor%20moveu%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20a%20posse%20para%20dentro%20da%20fun%C3%A7%C3%A3o%0A%7D%0A
//
// Notebook "retorno_e_escopo", exemplo 3/3 — "Return Values and Scope"
// (Listing 4-5), cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// Padrão para "usar e devolver": como passar `s` por valor move a posse
// para dentro da função, a única forma de recuperá-la é devolvê-la de
// volta — aqui, dentro de uma tupla junto do resultado.
fn main() {
    let s4 = String::from("hello");
    let (s5, tamanho) = calcula_tamanho(s4);
    println!("o tamanho de '{s5}' é {tamanho}");
}

fn calcula_tamanho(s: String) -> (String, usize) {
    let tamanho = s.len();
    (s, tamanho) // devolve `s` de volta, já que passá-la por valor moveu
                 // a posse para dentro da função
}
