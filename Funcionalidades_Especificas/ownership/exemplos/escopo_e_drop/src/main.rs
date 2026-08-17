// Baseado em "Variable Scope" (Listing 4-1) e "Memory and Allocation" —
// cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    // Uma variável só é válida dentro do escopo (bloco) em que foi
    // declarada — igual em Rust e em C.
    {
        let s = "hello"; // s é válida a partir daqui
        println!("dentro do escopo: {s}");
    } // o escopo termina aqui; s deixa de existir

    // A diferença aparece com dados alocados no heap. Quando uma `String`
    // sai de escopo, o Rust chama `drop` automaticamente e libera a
    // memória — sem coletor de lixo e sem free() manual.
    {
        let heap_s = String::from("hello"); // heap_s é válida a partir daqui
        println!("dentro do escopo (heap): {heap_s}");
    } // drop(heap_s) é chamado aqui; a memória do heap é liberada

    println!("fim do main: nenhuma das variáveis acima existe mais aqui");
}
