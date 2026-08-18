// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22move%22%2C%20exemplo%202%2F5%20%E2%80%94%20%22Ways%20Variables%20and%20Data%20Interact%3A%0A%2F%2F%20Move%22%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20Reatribuir%20uma%20vari%C3%A1vel%20dispara%20o%20%60drop%60%20do%20valor%20antigo%20na%20hora%2C%0A%2F%2F%20antes%20mesmo%20do%20fim%20do%20escopo.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20s3%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20println%21%28%22s3%20antes%20%3D%20%7Bs3%7D%22%29%3B%0A%20%20%20%20s3%20%3D%20String%3A%3Afrom%28%22ahoy%22%29%3B%20%2F%2F%20o%20%22hello%22%20antigo%20%C3%A9%20liberado%20agora%0A%20%20%20%20println%21%28%22s3%20depois%20%3D%20%7Bs3%7D%22%29%3B%0A%7D%0A
//
// Notebook "move", exemplo 2/5 — "Ways Variables and Data Interact:
// Move", cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// Reatribuir uma variável dispara o `drop` do valor antigo na hora,
// antes mesmo do fim do escopo.
fn main() {
    let mut s3 = String::from("hello");
    println!("s3 antes = {s3}");
    s3 = String::from("ahoy"); // o "hello" antigo é liberado agora
    println!("s3 depois = {s3}");
}
