// Duas referências de entrada, uma de saída: a elision não resolve
// sozinha, e sem anotação o compilador rejeita.

fn maior(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let resultado = maior(string1.as_str(), string2.as_str());
    println!("A maior string é {}", resultado);
}
