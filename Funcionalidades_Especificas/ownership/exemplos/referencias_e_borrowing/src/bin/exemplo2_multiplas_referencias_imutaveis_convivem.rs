// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_e_borrowing%22%2C%20exemplo%202%2F5%20%E2%80%94%20%22References%20and%0A%2F%2F%20Borrowing%22%2C%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20V%C3%A1rias%20refer%C3%AAncias%20imut%C3%A1veis%20para%20o%20mesmo%20dado%20podem%20conviver%20ao%20mesmo%0A%2F%2F%20tempo%3A%20nenhuma%20delas%20pode%20alterar%20o%20valor%2C%20e%20leitura%20simult%C3%A2nea%20n%C3%A3o%20%C3%A9%0A%2F%2F%20um%20data%20race%20%28contraste%20com%20%60referencias_mutaveis%60%2C%20onde%20s%C3%B3%20uma%0A%2F%2F%20refer%C3%AAncia%20MUT%C3%81VEL%20pode%20existir%20por%20vez%29.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20let%20r1%20%3D%20%26s%3B%0A%20%20%20%20let%20r2%20%3D%20%26s%3B%0A%20%20%20%20println%21%28%22r1%20%3D%20%7Br1%7D%2C%20r2%20%3D%20%7Br2%7D%2C%20e%20s%20ainda%20%C3%A9%20v%C3%A1lida%20%3D%20%7Bs%7D%22%29%3B%0A%7D%0A
//
// Notebook "referencias_e_borrowing", exemplo 2/5 — "References and
// Borrowing", cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// Várias referências imutáveis para o mesmo dado podem conviver ao mesmo
// tempo: nenhuma delas pode alterar o valor, e leitura simultânea não é
// um data race (contraste com `referencias_mutaveis`, onde só uma
// referência MUTÁVEL pode existir por vez).
fn main() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1 = {r1}, r2 = {r2}, e s ainda é válida = {s}");
}
