// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22move%22%2C%20exemplo%205%2F5%20%E2%80%94%20%22Stack-Only%20Data%3A%20Copy%22%2C%20cap.%204.1%20do%0A%2F%2F%20Rust%20Book%3A%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20A%20regra%20do%20Rust%20Book%20para%20tipos%20compostos%3A%20um%20tipo%20s%C3%B3%20%C3%A9%20%60Copy%60%20se%0A%2F%2F%20TODAS%20as%20suas%20partes%20forem%20%60Copy%60.%20Basta%20um%20peda%C3%A7o%20dono%20de%20dados%20no%0A%2F%2F%20heap%20para%20o%20tipo%20inteiro%20passar%20a%20ser%20movido.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20a%20%3D%20%281%2C%20true%2C%20%27x%27%29%3B%20%2F%2F%20s%C3%B3%20escalares%3A%20a%20tupla%20inteira%20%C3%A9%20Copy%0A%20%20%20%20let%20b%20%3D%20a%3B%0A%20%20%20%20println%21%28%22a%20%3D%20%7Ba%3A%3F%7D%2C%20b%20%3D%20%7Bb%3A%3F%7D%22%29%3B%20%2F%2F%20%60a%60%20continua%20v%C3%A1lida%0A%0A%20%20%20%20let%20c%20%3D%20%281%2C%20String%3A%3Afrom%28%22hello%22%29%29%3B%20%2F%2F%20cont%C3%A9m%20String%3A%20n%C3%A3o%20%C3%A9%20Copy%0A%20%20%20%20let%20d%20%3D%20c%3B%20%2F%2F%20move%0A%0A%20%20%20%20println%21%28%22d%20%3D%20%7Bd%3A%3F%7D%22%29%3B%0A%0A%20%20%20%20%2F%2F%20println%21%28%22c%20%3D%20%7Bc%3A%3F%7D%22%29%3B%20%2F%2F%20erro%5BE0382%5D%3A%20borrow%20of%20moved%20value%3A%20%60c%60%0A%7D%0A
//
// Notebook "move", exemplo 5/5 — "Stack-Only Data: Copy", cap. 4.1 do
// Rust Book: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// A regra do Rust Book para tipos compostos: um tipo só é `Copy` se
// TODAS as suas partes forem `Copy`. Basta um pedaço dono de dados no
// heap para o tipo inteiro passar a ser movido.
fn main() {
    let a = (1, true, 'x'); // só escalares: a tupla inteira é Copy
    let b = a;
    println!("a = {a:?}, b = {b:?}"); // `a` continua válida

    let c = (1, String::from("hello")); // contém String: não é Copy
    let d = c; // move

    println!("d = {d:?}");

    // println!("c = {c:?}"); // erro[E0382]: borrow of moved value: `c`
}
