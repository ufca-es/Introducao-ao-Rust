// A anotação <'a> amarra x, y e o retorno ao mesmo lifetime — o
// compilador consegue provar que a referência devolvida é válida.

fn maior<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let resultado = maior(string1.as_str(), string2.as_str());
    println!("A maior string é {}", resultado);
}

// Leitura: existe um lifetime 'a tal que x, y e o retorno vivem, no
// mínimo, durante 'a. O retorno fica amarrado ao MENOR dos dois
// lifetimes de entrada.
