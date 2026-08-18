// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22strings%22%2C%20exemplo%202%2F5%20%E2%80%94%20%22The%20String%20Type%22%2C%20cap.%204.1%20do%20Rust%0A%2F%2F%20Book%3A%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20Uma%20%60String%60%20pode%20crescer%20depois%20de%20criada.%20%60capacity%28%29%60%20deixa%20vis%C3%ADvel%0A%2F%2F%20que%20ela%20cresce%20aos%20saltos%2C%20e%20n%C3%A3o%20um%20byte%20por%20vez%3A%20cada%20salto%20%C3%A9%20uma%0A%2F%2F%20realoca%C3%A7%C3%A3o%2C%20ent%C3%A3o%20reservar%20mais%20que%20o%20necess%C3%A1rio%20%C3%A9%20o%20que%20evita%0A%2F%2F%20realocar%20a%20cada%20push.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20println%21%28%22s%20%3D%20%7Bs%7D%20%28len%20%3D%20%7B%7D%2C%20capacity%20%3D%20%7B%7D%29%22%2C%20s.len%28%29%2C%20s.capacity%28%29%29%3B%0A%0A%20%20%20%20s.push_str%28%22%2C%20world%22%29%3B%20%2F%2F%20s%C3%B3%20existe%20porque%20%60s%60%20%C3%A9%20String%2C%20n%C3%A3o%20%26str%0A%20%20%20%20s.push%28%27%21%27%29%3B%0A%20%20%20%20println%21%28%22s%20%3D%20%7Bs%7D%20%28len%20%3D%20%7B%7D%2C%20capacity%20%3D%20%7B%7D%29%22%2C%20s.len%28%29%2C%20s.capacity%28%29%29%3B%0A%7D%0A
//
// Notebook "strings", exemplo 2/5 — "The String Type", cap. 4.1 do Rust
// Book: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// Uma `String` pode crescer depois de criada. `capacity()` deixa visível
// que ela cresce aos saltos, e não um byte por vez: cada salto é uma
// realocação, então reservar mais que o necessário é o que evita
// realocar a cada push.
fn main() {
    let mut s = String::from("hello");
    println!("s = {s} (len = {}, capacity = {})", s.len(), s.capacity());

    s.push_str(", world"); // só existe porque `s` é String, não &str
    s.push('!');
    println!("s = {s} (len = {}, capacity = {})", s.len(), s.capacity());
}
