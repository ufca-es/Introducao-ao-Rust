// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_e_borrowing%22%2C%20exemplo%205%2F5%20%E2%80%94%20%22References%20and%0A%2F%2F%20Borrowing%22%2C%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20Enquanto%20uma%20refer%C3%AAncia%20estiver%20viva%2C%20o%20dado%20emprestado%20n%C3%A3o%20pode%20ser%0A%2F%2F%20movido%20%E2%80%94%20o%20move%20puxaria%20o%20buffer%20debaixo%20da%20refer%C3%AAncia.%20Depois%20do%0A%2F%2F%20%C3%BAltimo%20uso%20da%20refer%C3%AAncia%20o%20empr%C3%A9stimo%20acaba%2C%20e%20a%C3%AD%20o%20move%20%C3%A9%20liberado.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%0A%20%20%20%20let%20r%20%3D%20%26s%3B%0A%20%20%20%20println%21%28%22emprestada%3A%20%7Br%7D%22%29%3B%20%2F%2F%20%C3%BAltimo%20uso%20de%20%60r%60%3A%20o%20empr%C3%A9stimo%20acaba%20aqui%0A%0A%20%20%20%20let%20dona%20%3D%20s%3B%20%2F%2F%20ok%3A%20nenhuma%20refer%C3%AAncia%20viva%20impede%20o%20move%0A%20%20%20%20println%21%28%22movida%20para%3A%20%7Bdona%7D%22%29%3B%0A%0A%20%20%20%20%2F%2F%20Se%20%60r%60%20fosse%20usada%20DEPOIS%20do%20move%2C%20n%C3%A3o%20compilaria%3A%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20%20%20%20%20let%20r%20%3D%20%26s%3B%0A%20%20%20%20%2F%2F%20%20%20%20%20let%20dona%20%3D%20s%3B%0A%20%20%20%20%2F%2F%20%20%20%20%20println%21%28%22%7Br%7D%22%29%3B%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20erro%5BE0505%5D%3A%20cannot%20move%20out%20of%20%60s%60%20because%20it%20is%20borrowed%0A%7D%0A
//
// Notebook "referencias_e_borrowing", exemplo 5/5 — "References and
// Borrowing", cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// Enquanto uma referência estiver viva, o dado emprestado não pode ser
// movido — o move puxaria o buffer debaixo da referência. Depois do
// último uso da referência o empréstimo acaba, e aí o move é liberado.
fn main() {
    let s = String::from("hello");

    let r = &s;
    println!("emprestada: {r}"); // último uso de `r`: o empréstimo acaba aqui

    let dona = s; // ok: nenhuma referência viva impede o move
    println!("movida para: {dona}");

    // Se `r` fosse usada DEPOIS do move, não compilaria:
    //
    //     let r = &s;
    //     let dona = s;
    //     println!("{r}");
    //
    // erro[E0505]: cannot move out of `s` because it is borrowed
}
