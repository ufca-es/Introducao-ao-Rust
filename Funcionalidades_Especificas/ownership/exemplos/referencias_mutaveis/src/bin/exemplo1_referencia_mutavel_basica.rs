// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_mutaveis%22%2C%20exemplo%201%2F4%20%E2%80%94%20%22Mutable%20References%22%2C%0A%2F%2F%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20muda%28%26mut%20s%29%3B%20%2F%2F%20%60%26mut%20s%60%20empresta%20uma%20refer%C3%AAncia%20mut%C3%A1vel%0A%20%20%20%20println%21%28%22%7Bs%7D%22%29%3B%0A%7D%0A%0Afn%20muda%28algo%3A%20%26mut%20String%29%20%7B%0A%20%20%20%20algo.push_str%28%22%2C%20world%22%29%3B%0A%7D%0A
//
// Notebook "referencias_mutaveis", exemplo 1/4 — "Mutable References",
// cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

fn main() {
    let mut s = String::from("hello");
    muda(&mut s); // `&mut s` empresta uma referência mutável
    println!("{s}");
}

fn muda(algo: &mut String) {
    algo.push_str(", world");
}
