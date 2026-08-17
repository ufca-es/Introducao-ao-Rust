// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22escopo_e_drop%22%2C%20exemplo%202%2F3%20%E2%80%94%20%22Memory%20and%20Allocation%22%2C%0A%2F%2F%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20A%20diferen%C3%A7a%20aparece%20com%20dados%20alocados%20no%20heap.%20Quando%20uma%20%60String%60%0A%2F%2F%20sai%20de%20escopo%2C%20o%20Rust%20chama%20%60drop%60%20automaticamente%20e%20libera%20a%0A%2F%2F%20mem%C3%B3ria%20%E2%80%94%20sem%20coletor%20de%20lixo%20e%20sem%20free%28%29%20manual.%0Afn%20main%28%29%20%7B%0A%20%20%20%20%7B%0A%20%20%20%20%20%20%20%20let%20heap_s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%20%2F%2F%20heap_s%20%C3%A9%20v%C3%A1lida%20a%20partir%20daqui%0A%20%20%20%20%20%20%20%20println%21%28%22dentro%20do%20escopo%20%28heap%29%3A%20%7Bheap_s%7D%22%29%3B%0A%20%20%20%20%7D%20%2F%2F%20drop%28heap_s%29%20%C3%A9%20chamado%20aqui%3B%20a%20mem%C3%B3ria%20do%20heap%20%C3%A9%20liberada%0A%7D%0A
//
// Notebook "escopo_e_drop", exemplo 2/3 — "Memory and Allocation",
// cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// A diferença aparece com dados alocados no heap. Quando uma `String`
// sai de escopo, o Rust chama `drop` automaticamente e libera a
// memória — sem coletor de lixo e sem free() manual.
fn main() {
    {
        let heap_s = String::from("hello"); // heap_s é válida a partir daqui
        println!("dentro do escopo (heap): {heap_s}");
    } // drop(heap_s) é chamado aqui; a memória do heap é liberada
}
