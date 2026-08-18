// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22escopo_e_drop%22%2C%20exemplo%204%2F5%20%E2%80%94%20%22Memory%20and%20Allocation%22%2C%0A%2F%2F%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20%60drop%28v%29%60%20antecipa%20a%20libera%C3%A7%C3%A3o%2C%20sem%20esperar%20o%20fim%20do%20escopo%20%E2%80%94%20o%20mais%0A%2F%2F%20perto%20que%20o%20Rust%20chega%20do%20%60free%28%29%60%20manual%20de%20C.%20A%20diferen%C3%A7a%20%C3%A9%20que%0A%2F%2F%20%60drop%60%20TOMA%20A%20POSSE%20do%20valor%3A%20a%20vari%C3%A1vel%20fica%20inv%C3%A1lida%20a%20partir%20dali%2C%0A%2F%2F%20ent%C3%A3o%20n%C3%A3o%20h%C3%A1%20como%20us%C3%A1-la%20nem%20liber%C3%A1-la%20de%20novo.%0Astruct%20Rotulada%28%26%27static%20str%29%3B%0A%0Aimpl%20Drop%20for%20Rotulada%20%7B%0A%20%20%20%20fn%20drop%28%26mut%20self%29%20%7B%0A%20%20%20%20%20%20%20%20println%21%28%22dropping%20%7B%7D%22%2C%20self.0%29%3B%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20cedo%20%3D%20Rotulada%28%22cedo%22%29%3B%0A%20%20%20%20let%20tarde%20%3D%20Rotulada%28%22tarde%22%29%3B%0A%0A%20%20%20%20drop%28cedo%29%3B%20%2F%2F%20liberada%20aqui%2C%20e%20n%C3%A3o%20no%20fim%20do%20escopo%0A%20%20%20%20println%21%28%22%27cedo%27%20j%C3%A1%20foi%20descartada%3B%20%27tarde%27%20%28%7B%7D%29%20continua%20viva%22%2C%20tarde.0%29%3B%0A%0A%20%20%20%20%2F%2F%20println%21%28%22%7B%7D%22%2C%20cedo.0%29%3B%20%2F%2F%20erro%5BE0382%5D%3A%20borrow%20of%20moved%20value%3A%20%60cedo%60%0A%7D%20%2F%2F%20drop%28tarde%29%0A
//
// Notebook "escopo_e_drop", exemplo 4/5 — "Memory and Allocation",
// cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// `drop(v)` antecipa a liberação, sem esperar o fim do escopo — o mais
// perto que o Rust chega do `free()` manual de C. A diferença é que
// `drop` TOMA A POSSE do valor: a variável fica inválida a partir dali,
// então não há como usá-la nem liberá-la de novo.
struct Rotulada(&'static str);

impl Drop for Rotulada {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

fn main() {
    let cedo = Rotulada("cedo");
    let tarde = Rotulada("tarde");

    drop(cedo); // liberada aqui, e não no fim do escopo
    println!("'cedo' já foi descartada; 'tarde' ({}) continua viva", tarde.0);

    // println!("{}", cedo.0); // erro[E0382]: borrow of moved value: `cedo`
} // drop(tarde)
