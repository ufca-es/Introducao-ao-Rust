// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22retorno_e_escopo%22%2C%20exemplo%205%2F5%20%E2%80%94%20%22Return%20Values%20and%20Scope%22%0A%2F%2F%20%28Listing%204-4%29%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20Qual%20valor%20%C3%A9%20devolvido%20pode%20ser%20decidido%20em%20tempo%20de%20execu%C3%A7%C3%A3o%3A%20a%20posse%0A%2F%2F%20do%20escolhido%20se%20move%20para%20quem%20chamou%2C%20e%20o%20n%C3%A3o%20escolhido%20%C3%A9%20descartado%0A%2F%2F%20no%20fim%20da%20fun%C3%A7%C3%A3o.%20O%20compilador%20n%C3%A3o%20precisa%20saber%20qual%20%C3%A9%20qual.%0Astruct%20Rotulada%28%26%27static%20str%29%3B%0A%0Aimpl%20Drop%20for%20Rotulada%20%7B%0A%20%20%20%20fn%20drop%28%26mut%20self%29%20%7B%0A%20%20%20%20%20%20%20%20println%21%28%22dropping%20%7B%7D%22%2C%20self.0%29%3B%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20escolhida%20%3D%20escolhe%28true%29%3B%0A%20%20%20%20println%21%28%22main%20ficou%20com%3A%20%7B%7D%22%2C%20escolhida.0%29%3B%0A%7D%20%2F%2F%20drop%28escolhida%29%0A%0Afn%20escolhe%28primeira%3A%20bool%29%20-%3E%20Rotulada%20%7B%0A%20%20%20%20let%20a%20%3D%20Rotulada%28%22a%22%29%3B%0A%20%20%20%20let%20b%20%3D%20Rotulada%28%22b%22%29%3B%0A%0A%20%20%20%20if%20primeira%20%7B%20a%20%7D%20else%20%7B%20b%20%7D%0A%7D%20%2F%2F%20a%20rotulada%20N%C3%83O%20devolvida%20sai%20de%20escopo%20e%20%C3%A9%20descartada%20aqui%0A
//
// Notebook "retorno_e_escopo", exemplo 5/5 — "Return Values and Scope"
// (Listing 4-4), cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// Qual valor é devolvido pode ser decidido em tempo de execução: a posse
// do escolhido se move para quem chamou, e o não escolhido é descartado
// no fim da função. O compilador não precisa saber qual é qual.
struct Rotulada(&'static str);

impl Drop for Rotulada {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

fn main() {
    let escolhida = escolhe(true);
    println!("main ficou com: {}", escolhida.0);
} // drop(escolhida)

fn escolhe(primeira: bool) -> Rotulada {
    let a = Rotulada("a");
    let b = Rotulada("b");

    if primeira { a } else { b }
} // a rotulada NÃO devolvida sai de escopo e é descartada aqui
