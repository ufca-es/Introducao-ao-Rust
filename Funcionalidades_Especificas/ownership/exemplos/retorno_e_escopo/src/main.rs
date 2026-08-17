// Baseado em "Return Values and Scope" (Listing 4-4 e Listing 4-5) —
// cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    let s1 = da_posse(); // o valor de retorno se move para s1

    let s2 = String::from("hello"); // s2 entra em escopo

    let s3 = pega_e_devolve(s2); // s2 se move para a função, que move seu
                                  // retorno para s3

    println!("s1 = {s1}, s3 = {s3}");

    // Padrão para "usar e devolver": como passar `s` por valor move a
    // posse para dentro da função, a única forma de recuperá-la é
    // devolvê-la de volta — aqui, dentro de uma tupla junto do
    // resultado.
    let s4 = String::from("hello");
    let (s5, tamanho) = calcula_tamanho(s4);
    println!("o tamanho de '{s5}' é {tamanho}");
}

fn da_posse() -> String {
    let alguma_string = String::from("yours");
    alguma_string // é devolvida e se move para quem chamou
}

fn pega_e_devolve(uma_string: String) -> String {
    uma_string // é devolvida e se move para quem chamou
}

fn calcula_tamanho(s: String) -> (String, usize) {
    let tamanho = s.len();
    (s, tamanho) // devolve `s` de volta, já que passá-la por valor moveu
                 // a posse para dentro da função
}
