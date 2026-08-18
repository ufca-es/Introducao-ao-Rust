// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22retorno_e_escopo%22%2C%20exemplo%204%2F5%20%E2%80%94%20%22Return%20Values%20and%20Scope%22%0A%2F%2F%20%28Listing%204-4%29%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20A%20posse%20pode%20atravessar%20v%C3%A1rias%20fun%C3%A7%C3%B5es%20seguidas%3A%20cada%20retorno%20passa%20o%0A%2F%2F%20valor%20ao%20pr%C3%B3ximo%20dono%2C%20sem%20copiar%20o%20buffer%20do%20heap%20nenhuma%20vez.%20S%C3%B3%20o%0A%2F%2F%20dono%20final%20chama%20%60drop%60.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s%20%3D%20enfeita%28cria%28%29%29%3B%0A%20%20%20%20println%21%28%22%7Bs%7D%22%29%3B%0A%7D%20%2F%2F%20drop%28s%29%20%E2%80%94%20o%20%C3%BAnico%20drop%20de%20todo%20o%20percurso%0A%0Afn%20cria%28%29%20-%3E%20String%20%7B%0A%20%20%20%20String%3A%3Afrom%28%22hello%22%29%0A%7D%0A%0Afn%20enfeita%28mut%20s%3A%20String%29%20-%3E%20String%20%7B%0A%20%20%20%20s.push_str%28%22%2C%20world%22%29%3B%20%2F%2F%20muta%20porque%20recebeu%20a%20POSSE%2C%20n%C3%A3o%20um%20empr%C3%A9stimo%0A%20%20%20%20s%0A%7D%0A
//
// Notebook "retorno_e_escopo", exemplo 4/5 — "Return Values and Scope"
// (Listing 4-4), cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// A posse pode atravessar várias funções seguidas: cada retorno passa o
// valor ao próximo dono, sem copiar o buffer do heap nenhuma vez. Só o
// dono final chama `drop`.
fn main() {
    let s = enfeita(cria());
    println!("{s}");
} // drop(s) — o único drop de todo o percurso

fn cria() -> String {
    String::from("hello")
}

fn enfeita(mut s: String) -> String {
    s.push_str(", world"); // muta porque recebeu a POSSE, não um empréstimo
    s
}
