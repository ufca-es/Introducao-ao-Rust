// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22referencias_mutaveis%22%2C%20exemplo%205%2F5%20%E2%80%94%20%22Mutable%20References%22%2C%0A%2F%2F%20cap.%204.2%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-02-references-and-borrowing.html%0A%0A%2F%2F%20A%20sa%C3%ADda%20que%20o%20Rust%20Book%20d%C3%A1%20para%20a%20regra%201%3A%20duas%20refer%C3%AAncias%20mut%C3%A1veis%0A%2F%2F%20s%C3%A3o%20permitidas%20desde%20que%20n%C3%A3o%20estejam%20vivas%20AO%20MESMO%20TEMPO.%20Fechar%20o%0A%2F%2F%20bloco%20encerra%20o%20primeiro%20empr%C3%A9stimo.%20%28O%20exemplo%204%20chega%20ao%20mesmo%0A%2F%2F%20resultado%20pelo%20fim%20do%20%C3%BAltimo%20uso%2C%20sem%20precisar%20de%20bloco.%29%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%0A%20%20%20%20%7B%0A%20%20%20%20%20%20%20%20let%20r1%20%3D%20%26mut%20s%3B%0A%20%20%20%20%20%20%20%20r1.push_str%28%22%2C%20world%22%29%3B%0A%20%20%20%20%7D%20%2F%2F%20r1%20sai%20de%20escopo%20aqui%3B%20o%20empr%C3%A9stimo%20mut%C3%A1vel%20termina%0A%0A%20%20%20%20let%20r2%20%3D%20%26mut%20s%3B%20%2F%2F%20ok%3A%20n%C3%A3o%20h%C3%A1%20outra%20refer%C3%AAncia%20mut%C3%A1vel%20viva%0A%20%20%20%20r2.push%28%27%21%27%29%3B%0A%0A%20%20%20%20println%21%28%22%7Bs%7D%22%29%3B%0A%7D%0A
//
// Notebook "referencias_mutaveis", exemplo 5/5 — "Mutable References",
// cap. 4.2 do Rust Book:
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// A saída que o Rust Book dá para a regra 1: duas referências mutáveis
// são permitidas desde que não estejam vivas AO MESMO TEMPO. Fechar o
// bloco encerra o primeiro empréstimo. (O exemplo 4 chega ao mesmo
// resultado pelo fim do último uso, sem precisar de bloco.)
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(", world");
    } // r1 sai de escopo aqui; o empréstimo mutável termina

    let r2 = &mut s; // ok: não há outra referência mutável viva
    r2.push('!');

    println!("{s}");
}
