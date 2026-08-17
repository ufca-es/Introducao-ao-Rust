// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22strings%22%2C%20exemplo%202%2F3%20%E2%80%94%20%22The%20String%20Type%22%2C%20cap.%204.1%20do%20Rust%0A%2F%2F%20Book%3A%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20Uma%20%60String%60%20pode%20crescer%20depois%20de%20criada%20--%20%60push_str%60%20e%20%60push%60%0A%2F%2F%20podem%20for%C3%A7ar%20uma%20realoca%C3%A7%C3%A3o%20por%20baixo%20dos%20panos%20quando%20a%20capacidade%0A%2F%2F%20atual%20n%C3%A3o%20%C3%A9%20suficiente.%20%60capacity%28%29%60%20deixa%20isso%20vis%C3%ADvel%3A%20ela%20cresce%20aos%0A%2F%2F%20saltos%2C%20n%C3%A3o%20um%20byte%20de%20cada%20vez%2C%20para%20evitar%20realocar%20a%20cada%20push.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20println%21%28%22s%20%3D%20%7Bs%7D%20%28len%20%3D%20%7B%7D%2C%20capacity%20%3D%20%7B%7D%29%22%2C%20s.len%28%29%2C%20s.capacity%28%29%29%3B%0A%0A%20%20%20%20s.push_str%28%22%2C%20world%22%29%3B%20%2F%2F%20s%C3%B3%20existe%20porque%20%60s%60%20%C3%A9%20uma%20String%2C%20n%C3%A3o%20%26str%0A%20%20%20%20s.push%28%27%21%27%29%3B%0A%20%20%20%20println%21%28%22s%20%3D%20%7Bs%7D%20%28len%20%3D%20%7B%7D%2C%20capacity%20%3D%20%7B%7D%29%22%2C%20s.len%28%29%2C%20s.capacity%28%29%29%3B%0A%7D%0A
//
// Notebook "strings", exemplo 2/3 — "The String Type", cap. 4.1 do Rust
// Book: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// Uma `String` pode crescer depois de criada -- `push_str` e `push`
// podem forçar uma realocação por baixo dos panos quando a capacidade
// atual não é suficiente. `capacity()` deixa isso visível: ela cresce aos
// saltos, não um byte de cada vez, para evitar realocar a cada push.
fn main() {
    let mut s = String::from("hello");
    println!("s = {s} (len = {}, capacity = {})", s.len(), s.capacity());

    s.push_str(", world"); // só existe porque `s` é uma String, não &str
    s.push('!');
    println!("s = {s} (len = {}, capacity = {})", s.len(), s.capacity());
}
