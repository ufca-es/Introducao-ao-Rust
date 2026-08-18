// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22move%22%2C%20exemplo%203%2F5%20%E2%80%94%20%22Stack-Only%20Data%3A%20Copy%22%2C%20cap.%204.1%20do%0A%2F%2F%20Rust%20Book%3A%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20Tipos%20simples%20e%20de%20tamanho%20fixo%20%28como%20i32%29%20implementam%20o%20trait%20%60Copy%60%3A%0A%2F%2F%20vivem%20s%C3%B3%20na%20stack%20e%20s%C3%A3o%20copiados%20trivialmente%20na%20atribui%C3%A7%C3%A3o%20%E2%80%94%20n%C3%A3o%20h%C3%A1%0A%2F%2F%20move%20nem%20necessidade%20de%20clone%28%29.%20Contraste%20direto%20com%20o%20exemplo%201%3A%20l%C3%A1%0A%2F%2F%20a%20atribui%C3%A7%C3%A3o%20invalida%20o%20original%3B%20aqui%2C%20n%C3%A3o.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20x%20%3D%205%3B%0A%20%20%20%20let%20y%20%3D%20x%3B%0A%20%20%20%20println%21%28%22x%20%3D%20%7Bx%7D%2C%20y%20%3D%20%7By%7D%22%29%3B%0A%7D%0A
//
// Notebook "move", exemplo 3/5 — "Stack-Only Data: Copy", cap. 4.1 do
// Rust Book: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// Tipos simples e de tamanho fixo (como i32) implementam o trait `Copy`:
// vivem só na stack e são copiados trivialmente na atribuição — não há
// move nem necessidade de clone(). Contraste direto com o exemplo 1: lá
// a atribuição invalida o original; aqui, não.
fn main() {
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
}
