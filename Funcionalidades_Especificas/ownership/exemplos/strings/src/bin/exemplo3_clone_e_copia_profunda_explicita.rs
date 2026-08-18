// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22strings%22%2C%20exemplo%203%2F5%20%E2%80%94%20%22Variables%20and%20Data%20Interacting%20with%0A%2F%2F%20Clone%22%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20clone%28%29%3A%20c%C3%B3pia%20profunda%20e%20EXPL%C3%8DCITA%20dos%20dados%20do%20heap.%20Mais%20cara%20que%20um%0A%2F%2F%20move%20%28duplica%20o%20buffer%20inteiro%29%2C%20mas%20mant%C3%A9m%20os%20dois%20nomes%20v%C3%A1lidos%20%E2%80%94%0A%2F%2F%20diferente%20de%20%60let%20s2%20%3D%20s1%3B%60%2C%20que%20move%20a%20posse%20e%20invalida%20%60s1%60%20%28veja%20o%0A%2F%2F%20notebook%20%60move%60%29.%20%60String%60%20n%C3%A3o%20implementa%20%60Copy%60%20justamente%20porque%0A%2F%2F%20duplicar%20um%20buffer%20de%20tamanho%20desconhecido%20a%20cada%20atribui%C3%A7%C3%A3o%20seria%20caro%0A%2F%2F%20demais%20para%20acontecer%20por%20baixo%20dos%20panos.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s1%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20let%20s2%20%3D%20s1.clone%28%29%3B%0A%20%20%20%20println%21%28%22s1%20%3D%20%7Bs1%7D%2C%20s2%20%3D%20%7Bs2%7D%22%29%3B%0A%7D%0A
//
// Notebook "strings", exemplo 3/5 — "Variables and Data Interacting with
// Clone", cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// clone(): cópia profunda e EXPLÍCITA dos dados do heap. Mais cara que um
// move (duplica o buffer inteiro), mas mantém os dois nomes válidos —
// diferente de `let s2 = s1;`, que move a posse e invalida `s1` (veja o
// notebook `move`). `String` não implementa `Copy` justamente porque
// duplicar um buffer de tamanho desconhecido a cada atribuição seria caro
// demais para acontecer por baixo dos panos.
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
}
