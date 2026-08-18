// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22escopo_e_drop%22%2C%20exemplo%205%2F5%20%E2%80%94%20%22Memory%20and%20Allocation%22%2C%0A%2F%2F%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20Sair%20de%20escopo%20n%C3%A3o%20%C3%A9%20o%20que%20descarta%20um%20valor%3A%20o%20que%20descarta%20%C3%A9%20o%20fim%0A%2F%2F%20da%20POSSE.%20Um%20valor%20criado%20dentro%20de%20um%20bloco%20sobrevive%20a%20ele%20se%20a%0A%2F%2F%20posse%20for%20movida%20para%20fora.%0Astruct%20Rotulada%28%26%27static%20str%29%3B%0A%0Aimpl%20Drop%20for%20Rotulada%20%7B%0A%20%20%20%20fn%20drop%28%26mut%20self%29%20%7B%0A%20%20%20%20%20%20%20%20println%21%28%22dropping%20%7B%7D%22%2C%20self.0%29%3B%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20sobrevivente%20%3D%20%7B%0A%20%20%20%20%20%20%20%20let%20interna%20%3D%20Rotulada%28%22interna%22%29%3B%0A%20%20%20%20%20%20%20%20let%20movida%20%3D%20Rotulada%28%22movida%22%29%3B%0A%20%20%20%20%20%20%20%20println%21%28%22dentro%20do%20bloco%3A%20%7B%7D%20e%20%7B%7D%22%2C%20interna.0%2C%20movida.0%29%3B%0A%20%20%20%20%20%20%20%20movida%20%2F%2F%20a%20posse%20sai%20do%20bloco...%0A%20%20%20%20%7D%3B%20%2F%2F%20...ent%C3%A3o%20s%C3%B3%20%60interna%60%20%C3%A9%20descartada%20aqui%0A%0A%20%20%20%20println%21%28%22fora%20do%20bloco%2C%20ainda%20viva%3A%20%7B%7D%22%2C%20sobrevivente.0%29%3B%0A%7D%20%2F%2F%20drop%28sobrevivente%29%20%E2%80%94%20o%20valor%20%22movida%22%2C%20agora%20com%20outro%20dono%0A
//
// Notebook "escopo_e_drop", exemplo 5/5 — "Memory and Allocation",
// cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// Sair de escopo não é o que descarta um valor: o que descarta é o fim
// da POSSE. Um valor criado dentro de um bloco sobrevive a ele se a
// posse for movida para fora.
struct Rotulada(&'static str);

impl Drop for Rotulada {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

fn main() {
    let sobrevivente = {
        let interna = Rotulada("interna");
        let movida = Rotulada("movida");
        println!("dentro do bloco: {} e {}", interna.0, movida.0);
        movida // a posse sai do bloco...
    }; // ...então só `interna` é descartada aqui

    println!("fora do bloco, ainda viva: {}", sobrevivente.0);
} // drop(sobrevivente) — o valor "movida", agora com outro dono
