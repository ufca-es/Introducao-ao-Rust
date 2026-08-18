// com_anotacao.rs
// Demonstra: a anotação <'a> amarra os lifetimes de entrada e saída,
// permitindo ao compilador provar que a referência devolvida é válida.
//
// Compile com: rustc com_anotacao.rs
// Rode com:    ./com_anotacao   (ou .\com_anotacao no PowerShell)

fn maior<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let resultado = maior(string1.as_str(), string2.as_str());
    println!("A maior string é {}", resultado);
}

// Leitura da assinatura: existe um lifetime 'a tal que x, y e o valor
// de retorno vivem, no mínimo, durante 'a. Isso amarra o retorno ao
// MENOR dos dois lifetimes de entrada.
