// Baseado em "Ownership and Functions" (Listing 4-3) — cap. 4.1 do Rust
// Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    let s = String::from("hello"); // s entra em escopo

    toma_posse(s); // o valor de s se move para dentro da função...
                   // ...e por isso não é mais válido aqui

    // println!("{s}"); // não compilaria: valor de s já foi movido

    let x = 5; // x entra em escopo

    faz_copia(x); // como i32 implementa Copy, x não se move para dentro
                  // da função, então ainda é válido usá-lo depois

    println!("x ainda pode ser usado depois de faz_copia: {x}");
} // aqui x sai de escopo, depois s; como o valor de s já foi movido,
  // nada de especial acontece com ele

fn toma_posse(alguma_string: String) { // alguma_string entra em escopo
    println!("{alguma_string}");
} // aqui alguma_string sai de escopo e `drop` é chamado; a memória é liberada

fn faz_copia(algum_inteiro: i32) { // algum_inteiro entra em escopo
    println!("{algum_inteiro}");
} // aqui algum_inteiro sai de escopo; nada de especial acontece
