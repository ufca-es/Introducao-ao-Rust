// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22ownership_em_funcoes%22%2C%20exemplo%202%2F2%20%E2%80%94%20%22Ownership%20and%0A%2F%2F%20Functions%22%20%28Listing%204-3%29%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20x%20%3D%205%3B%20%2F%2F%20x%20entra%20em%20escopo%0A%0A%20%20%20%20faz_copia%28x%29%3B%20%2F%2F%20como%20i32%20implementa%20Copy%2C%20x%20n%C3%A3o%20se%20move%20para%20dentro%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20da%20fun%C3%A7%C3%A3o%2C%20ent%C3%A3o%20ainda%20%C3%A9%20v%C3%A1lido%20us%C3%A1-lo%20depois%0A%0A%20%20%20%20println%21%28%22x%20ainda%20pode%20ser%20usado%20depois%20de%20faz_copia%3A%20%7Bx%7D%22%29%3B%0A%7D%0A%0Afn%20faz_copia%28algum_inteiro%3A%20i32%29%20%7B%20%2F%2F%20algum_inteiro%20entra%20em%20escopo%0A%20%20%20%20println%21%28%22%7Balgum_inteiro%7D%22%29%3B%0A%7D%20%2F%2F%20aqui%20algum_inteiro%20sai%20de%20escopo%3B%20nada%20de%20especial%20acontece%0A
//
// Notebook "ownership_em_funcoes", exemplo 2/2 — "Ownership and
// Functions" (Listing 4-3), cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    let x = 5; // x entra em escopo

    faz_copia(x); // como i32 implementa Copy, x não se move para dentro
                  // da função, então ainda é válido usá-lo depois

    println!("x ainda pode ser usado depois de faz_copia: {x}");
}

fn faz_copia(algum_inteiro: i32) { // algum_inteiro entra em escopo
    println!("{algum_inteiro}");
} // aqui algum_inteiro sai de escopo; nada de especial acontece
