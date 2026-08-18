// Contraste com o exemplo 1: com UMA referência de entrada só, a
// segunda regra de elision resolve sozinha — o retorno herda o
// lifetime do único parâmetro, sem precisar anotar nada.

fn primeira_palavra(s: &str) -> &str {
    match s.find(' ') {
        Some(fim) => &s[0..fim],
        None => s,
    }
}

fn main() {
    let frase = String::from("segurança de memória sem garbage collector");
    println!("Primeira palavra: {}", primeira_palavra(&frase));
}
