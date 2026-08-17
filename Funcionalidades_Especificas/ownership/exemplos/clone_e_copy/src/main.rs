// Baseado em "Variables and Data Interacting with Clone" e "Stack-Only
// Data: Copy" — cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    // clone(): cópia profunda e EXPLÍCITA dos dados do heap. Mais cara
    // que um move, mas mantém os dois nomes válidos.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // Copy: tipos simples e de tamanho fixo (como i32) são copiados
    // trivialmente na atribuição — não há move nem necessidade de
    // clone().
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
}
