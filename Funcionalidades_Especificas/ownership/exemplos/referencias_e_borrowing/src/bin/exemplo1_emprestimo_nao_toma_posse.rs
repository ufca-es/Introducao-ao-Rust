// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_e_borrowing%22%2C%20exemplo%201%2F5%20%E2%80%94%20%22References%20and%0A%2F%2F%20Borrowing%22%2C%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20Diferente%20de%20%60ownership_em_funcoes%60%20%28onde%20passar%20uma%20%60String%60%20por%20valor%0A%2F%2F%20move%20a%20posse%20para%20dentro%20da%20fun%C3%A7%C3%A3o%29%2C%20aqui%20%60%26s1%60%20empresta%20uma%20refer%C3%AAncia%0A%2F%2F%20sem%20tomar%20posse%20%E2%80%94%20ent%C3%A3o%20%60s1%60%20continua%20v%C3%A1lida%20depois%20da%20chamada.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s1%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%0A%20%20%20%20let%20tamanho%20%3D%20calcula_tamanho%28%26s1%29%3B%20%2F%2F%20%60%26s1%60%20empresta%2C%20n%C3%A3o%20move%0A%0A%20%20%20%20println%21%28%22O%20tamanho%20de%20%27%7Bs1%7D%27%20%C3%A9%20%7Btamanho%7D.%22%29%3B%0A%0A%20%20%20%20%2F%2F%20Refer%C3%AAncias%20s%C3%A3o%20imut%C3%A1veis%20por%20padr%C3%A3o%2C%20assim%20como%20as%20vari%C3%A1veis%0A%20%20%20%20%2F%2F%20comuns.%20A%20fun%C3%A7%C3%A3o%20abaixo%20N%C3%83O%20compila%3A%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20%20%20%20%20fn%20muda%28algo%3A%20%26String%29%20%7B%0A%20%20%20%20%2F%2F%20%20%20%20%20%20%20%20%20algo.push_str%28%22%2C%20world%22%29%3B%0A%20%20%20%20%2F%2F%20%20%20%20%20%7D%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20erro%5BE0596%5D%3A%20cannot%20borrow%20%60%2Aalgo%60%20as%20mutable%2C%20as%20it%20is%20behind%20a%20%60%26%60%0A%20%20%20%20%2F%2F%20reference%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20Para%20mutar%20atrav%C3%A9s%20de%20uma%20refer%C3%AAncia%2C%20ela%20precisa%20ser%20%60%26mut%60%20%28veja%0A%20%20%20%20%2F%2F%20o%20notebook%20%60referencias_mutaveis%60%29.%0A%7D%0A%0Afn%20calcula_tamanho%28s%3A%20%26String%29%20-%3E%20usize%20%7B%0A%20%20%20%20s.len%28%29%0A%7D%20%2F%2F%20s%20sai%20de%20escopo%2C%20mas%20n%C3%A3o%20%C3%A9%20dona%20do%20dado%3A%20nada%20%C3%A9%20descartado%0A
//
// Notebook "referencias_e_borrowing", exemplo 1/5 — "References and
// Borrowing", cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// Diferente de `ownership_em_funcoes` (onde passar uma `String` por valor
// move a posse para dentro da função), aqui `&s1` empresta uma referência
// sem tomar posse — então `s1` continua válida depois da chamada.
fn main() {
    let s1 = String::from("hello");

    let tamanho = calcula_tamanho(&s1); // `&s1` empresta, não move

    println!("O tamanho de '{s1}' é {tamanho}.");

    // Referências são imutáveis por padrão, assim como as variáveis
    // comuns. A função abaixo NÃO compila:
    //
    //     fn muda(algo: &String) {
    //         algo.push_str(", world");
    //     }
    //
    // erro[E0596]: cannot borrow `*algo` as mutable, as it is behind a `&`
    // reference
    //
    // Para mutar através de uma referência, ela precisa ser `&mut` (veja
    // o notebook `referencias_mutaveis`).
}

fn calcula_tamanho(s: &String) -> usize {
    s.len()
} // s sai de escopo, mas não é dona do dado: nada é descartado
