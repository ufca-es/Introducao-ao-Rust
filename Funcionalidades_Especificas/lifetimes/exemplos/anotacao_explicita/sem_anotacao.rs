// sem_anotacao.rs
// Demonstra: quando a elision não resolve sozinha, o compilador rejeita
// e pede a anotação de lifetime.
//
// Compile com: rustc sem_anotacao.rs

fn maior(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let resultado = maior(string1.as_str(), string2.as_str());
    println!("A maior string é {}", resultado);
}

/*
error[E0106]: missing lifetime specifier
 --> sem_anotacao.rs:7:31
  |
7 | fn maior(x: &str, y: &str) -> &str {
  |             ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the
    signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
7 | fn maior<'a>(x: &'a str, y: &'a str) -> &'a str {
  |          ++++     ++         ++          ++
*/
