// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_e_borrowing%22%2C%20exemplo%204%2F5%20%E2%80%94%20%22References%20and%0A%2F%2F%20Borrowing%22%2C%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20Uma%20refer%C3%AAncia%20%C3%A9%20um%20ponteiro%2C%20e%20%60%2A%60%20%C3%A9%20o%20operador%20que%20acessa%20o%20valor%0A%2F%2F%20apontado.%20Em%20chamadas%20de%20m%C3%A9todo%20%28%60s.len%28%29%60%29%20o%20Rust%20desreferencia%0A%2F%2F%20sozinho%2C%20mas%20em%20opera%C3%A7%C3%B5es%20sobre%20o%20valor%20em%20si%20%E2%80%94%20comparar%2C%20somar%20%E2%80%94%20o%0A%2F%2F%20%60%2A%60%20precisa%20ser%20escrito.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20x%20%3D%205%3B%0A%20%20%20%20let%20r%20%3D%20%26x%3B%0A%0A%20%20%20%20println%21%28%22r%20aponta%20para%20%7B%7D%22%2C%20%2Ar%29%3B%0A%20%20%20%20println%21%28%22%2Ar%20%3D%3D%20x%3F%20%7B%7D%22%2C%20%2Ar%20%3D%3D%20x%29%3B%0A%20%20%20%20println%21%28%22%2Ar%20%2B%201%20%3D%20%7B%7D%22%2C%20%2Ar%20%2B%201%29%3B%0A%0A%20%20%20%20println%21%28%22x%20continua%20v%C3%A1lida%3A%20%7Bx%7D%22%29%3B%20%2F%2F%20emprestar%20nunca%20invalida%20o%20dono%0A%7D%0A
//
// Notebook "referencias_e_borrowing", exemplo 4/5 — "References and
// Borrowing", cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// Uma referência é um ponteiro, e `*` é o operador que acessa o valor
// apontado. Em chamadas de método (`s.len()`) o Rust desreferencia
// sozinho, mas em operações sobre o valor em si — comparar, somar — o
// `*` precisa ser escrito.
fn main() {
    let x = 5;
    let r = &x;

    println!("r aponta para {}", *r);
    println!("*r == x? {}", *r == x);
    println!("*r + 1 = {}", *r + 1);

    println!("x continua válida: {x}"); // emprestar nunca invalida o dono
}
