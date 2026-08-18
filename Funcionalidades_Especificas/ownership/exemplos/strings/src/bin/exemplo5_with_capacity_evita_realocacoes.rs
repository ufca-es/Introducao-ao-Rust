// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22strings%22%2C%20exemplo%205%2F5%20%E2%80%94%20%22Memory%20and%20Allocation%22%2C%20cap.%204.1%20do%0A%2F%2F%20Rust%20Book%3A%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20Consequ%C3%AAncia%20pr%C3%A1tica%20do%20exemplo%202%3A%20cada%20salto%20de%20capacidade%20%C3%A9%20uma%0A%2F%2F%20realoca%C3%A7%C3%A3o%20%28aloca%20um%20buffer%20maior%20e%20copia%20o%20conte%C3%BAdo%29.%20Quando%20o%0A%2F%2F%20tamanho%20final%20j%C3%A1%20%C3%A9%20conhecido%2C%20%60with_capacity%60%20reserva%20tudo%20de%20uma%20vez%0A%2F%2F%20e%20nenhuma%20realoca%C3%A7%C3%A3o%20acontece.%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20sem_reserva%20%3D%20String%3A%3Anew%28%29%3B%0A%20%20%20%20let%20mut%20com_reserva%20%3D%20String%3A%3Awith_capacity%2813%29%3B%0A%0A%20%20%20%20for%20_%20in%200..13%20%7B%0A%20%20%20%20%20%20%20%20sem_reserva.push%28%27x%27%29%3B%0A%20%20%20%20%20%20%20%20com_reserva.push%28%27x%27%29%3B%0A%20%20%20%20%7D%0A%0A%20%20%20%20println%21%28%22sem%20reserva%3A%20len%20%3D%20%7B%7D%2C%20capacity%20%3D%20%7B%7D%22%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20sem_reserva.len%28%29%2C%20sem_reserva.capacity%28%29%29%3B%0A%20%20%20%20println%21%28%22com%20reserva%3A%20len%20%3D%20%7B%7D%2C%20capacity%20%3D%20%7B%7D%22%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20com_reserva.len%28%29%2C%20com_reserva.capacity%28%29%29%3B%0A%7D%0A
//
// Notebook "strings", exemplo 5/5 — "Memory and Allocation", cap. 4.1 do
// Rust Book: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// Consequência prática do exemplo 2: cada salto de capacidade é uma
// realocação (aloca um buffer maior e copia o conteúdo). Quando o
// tamanho final já é conhecido, `with_capacity` reserva tudo de uma vez
// e nenhuma realocação acontece.
fn main() {
    let mut sem_reserva = String::new();
    let mut com_reserva = String::with_capacity(13);

    for _ in 0..13 {
        sem_reserva.push('x');
        com_reserva.push('x');
    }

    println!("sem reserva: len = {}, capacity = {}",
             sem_reserva.len(), sem_reserva.capacity());
    println!("com reserva: len = {}, capacity = {}",
             com_reserva.len(), com_reserva.capacity());
}
