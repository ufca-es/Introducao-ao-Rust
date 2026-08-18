// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22ownership_em_funcoes%22%2C%20exemplo%205%2F5%20%E2%80%94%20%22Ownership%20and%0A%2F%2F%20Functions%22%20%28Listing%204-3%29%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20Mover%20ou%20copiar%20n%C3%A3o%20%C3%A9%20uma%20decis%C3%A3o%20da%20chamada%2C%20e%20sim%20do%20TIPO%20de%20cada%0A%2F%2F%20argumento%3A%20na%20mesma%20chamada%2C%20o%20%60i32%60%20%C3%A9%20copiado%20e%20a%20%60String%60%20%C3%A9%20movida.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20texto%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20let%20numero%20%3D%205%3B%0A%0A%20%20%20%20mistura%28texto%2C%20numero%29%3B%0A%0A%20%20%20%20println%21%28%22numero%20foi%20copiado%20e%20continua%20v%C3%A1lido%3A%20%7Bnumero%7D%22%29%3B%0A%0A%20%20%20%20%2F%2F%20println%21%28%22%7Btexto%7D%22%29%3B%20%2F%2F%20erro%5BE0382%5D%3A%20borrow%20of%20moved%20value%3A%20%60texto%60%0A%7D%0A%0Afn%20mistura%28dono%3A%20String%2C%20copia%3A%20i32%29%20%7B%0A%20%20%20%20println%21%28%22dono%20%3D%20%7Bdono%7D%2C%20copia%20%3D%20%7Bcopia%7D%22%29%3B%0A%7D%20%2F%2F%20%60dono%60%20%C3%A9%20liberada%20aqui%3B%20%60copia%60%20s%C3%B3%20some%20da%20stack%0A
//
// Notebook "ownership_em_funcoes", exemplo 5/5 — "Ownership and
// Functions" (Listing 4-3), cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// Mover ou copiar não é uma decisão da chamada, e sim do TIPO de cada
// argumento: na mesma chamada, o `i32` é copiado e a `String` é movida.
fn main() {
    let texto = String::from("hello");
    let numero = 5;

    mistura(texto, numero);

    println!("numero foi copiado e continua válido: {numero}");

    // println!("{texto}"); // erro[E0382]: borrow of moved value: `texto`
}

fn mistura(dono: String, copia: i32) {
    println!("dono = {dono}, copia = {copia}");
} // `dono` é liberada aqui; `copia` só some da stack
