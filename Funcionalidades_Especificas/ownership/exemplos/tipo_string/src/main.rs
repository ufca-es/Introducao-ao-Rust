// Baseado na seção "The String Type" — cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    // Literal de string (&str): tamanho conhecido em tempo de compilação,
    // embutido no binário, sempre imutável.
    let literal = "hello";
    println!("literal: {literal}");

    // `String::from` aloca no heap: pode crescer e ser mutada, desde que
    // a variável seja declarada com `mut`.
    let mut dinamica = String::from("hello");
    dinamica.push_str(", world!"); // só existe porque `dinamica` é uma String
    println!("dinamica: {dinamica}");
}
