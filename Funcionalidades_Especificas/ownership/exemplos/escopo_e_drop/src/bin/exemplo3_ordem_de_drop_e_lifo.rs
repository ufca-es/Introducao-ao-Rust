// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22escopo_e_drop%22%2C%20exemplo%203%2F5%20%E2%80%94%20%22Memory%20and%20Allocation%22%2C%0A%2F%2F%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20O%20%60drop%60%20autom%C3%A1tico%20de%20v%C3%A1rios%20valores%20no%20mesmo%20escopo%20segue%20a%20ordem%0A%2F%2F%20LIFO%20da%20pilha%3A%20o%20%C3%BAltimo%20a%20entrar%20em%20escopo%20%C3%A9%20o%20primeiro%20a%20ser%0A%2F%2F%20descartado.%0Astruct%20Rotulada%28%26%27static%20str%29%3B%0A%0Aimpl%20Drop%20for%20Rotulada%20%7B%0A%20%20%20%20fn%20drop%28%26mut%20self%29%20%7B%0A%20%20%20%20%20%20%20%20println%21%28%22dropping%20%7B%7D%22%2C%20self.0%29%3B%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20_a%20%3D%20Rotulada%28%22a%22%29%3B%0A%20%20%20%20let%20_b%20%3D%20Rotulada%28%22b%22%29%3B%0A%20%20%20%20let%20_c%20%3D%20Rotulada%28%22c%22%29%3B%0A%20%20%20%20println%21%28%22fim%20do%20escopo%3A%20os%20drops%20devem%20imprimir%20na%20ordem%20c%2C%20b%2C%20a%22%29%3B%0A%7D%20%2F%2F%20drop%28_c%29%2C%20drop%28_b%29%2C%20drop%28_a%29%20%E2%80%94%20nessa%20ordem%0A
//
// Notebook "escopo_e_drop", exemplo 3/5 — "Memory and Allocation",
// cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// O `drop` automático de vários valores no mesmo escopo segue a ordem
// LIFO da pilha: o último a entrar em escopo é o primeiro a ser
// descartado.
struct Rotulada(&'static str);

impl Drop for Rotulada {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

fn main() {
    let _a = Rotulada("a");
    let _b = Rotulada("b");
    let _c = Rotulada("c");
    println!("fim do escopo: os drops devem imprimir na ordem c, b, a");
} // drop(_c), drop(_b), drop(_a) — nessa ordem
