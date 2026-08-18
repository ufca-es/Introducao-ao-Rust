// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_e_borrowing%22%2C%20exemplo%203%2F5%20%E2%80%94%20%22References%20and%0A%2F%2F%20Borrowing%22%2C%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20Um%20empr%C3%A9stimo%20pode%20ser%20repassado%20de%20fun%C3%A7%C3%A3o%20em%20fun%C3%A7%C3%A3o%20%C3%A0%20vontade%2C%20porque%0A%2F%2F%20nenhuma%20delas%20toma%20posse.%20O%20dono%20original%20continua%20sendo%20o%20%C3%BAnico%0A%2F%2F%20respons%C3%A1vel%20por%20liberar%20o%20dado%20%E2%80%94%20e%20o%20%C3%BAnico%20lugar%20onde%20%60drop%60%20acontece.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%0A%20%20%20%20println%21%28%22primeira%20letra%20de%20%27%7Bs%7D%27%3A%20%7B%7D%22%2C%20primeira_letra%28%26s%29%29%3B%0A%7D%20%2F%2F%20drop%28s%29%20acontece%20aqui%2C%20no%20dono%0A%0Afn%20primeira_letra%28s%3A%20%26String%29%20-%3E%20char%20%7B%0A%20%20%20%20inicial%28s%29%20%2F%2F%20repassa%20o%20MESMO%20empr%C3%A9stimo%20adiante%0A%7D%0A%0Afn%20inicial%28s%3A%20%26String%29%20-%3E%20char%20%7B%0A%20%20%20%20s.chars%28%29.next%28%29.unwrap_or%28%27%3F%27%29%0A%7D%0A
//
// Notebook "referencias_e_borrowing", exemplo 3/5 — "References and
// Borrowing", cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// Um empréstimo pode ser repassado de função em função à vontade, porque
// nenhuma delas toma posse. O dono original continua sendo o único
// responsável por liberar o dado — e o único lugar onde `drop` acontece.
fn main() {
    let s = String::from("hello");

    println!("primeira letra de '{s}': {}", primeira_letra(&s));
} // drop(s) acontece aqui, no dono

fn primeira_letra(s: &String) -> char {
    inicial(s) // repassa o MESMO empréstimo adiante
}

fn inicial(s: &String) -> char {
    s.chars().next().unwrap_or('?')
}
