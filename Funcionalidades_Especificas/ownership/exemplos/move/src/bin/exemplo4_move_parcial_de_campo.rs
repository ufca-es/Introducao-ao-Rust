// Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Notebook%20%22move%22%2C%20exemplo%204%2F5%20%E2%80%94%20%22Ways%20Variables%20and%20Data%20Interact%3A%0A%2F%2F%20Move%22%2C%20cap.%204.1%20do%20Rust%20Book%3A%0A%2F%2F%20https%3A%2F%2Fdoc.rust-lang.org%2Fbook%2Fch04-01-what-is-ownership.html%0A%0A%2F%2F%20A%20posse%20%C3%A9%20rastreada%20campo%20a%20campo%3A%20mover%20um%20campo%20para%20fora%20de%20uma%0A%2F%2F%20struct%20%C3%A9%20um%20MOVE%20PARCIAL.%20S%C3%B3%20aquele%20campo%20%C3%A9%20invalidado%20%E2%80%94%20os%20outros%0A%2F%2F%20continuam%20acess%C3%ADveis%2C%20mas%20a%20struct%20j%C3%A1%20n%C3%A3o%20pode%20ser%20usada%20como%20um%20todo.%0Astruct%20Pessoa%20%7B%0A%20%20%20%20nome%3A%20String%2C%0A%20%20%20%20idade%3A%20u32%2C%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20p%20%3D%20Pessoa%20%7B%0A%20%20%20%20%20%20%20%20nome%3A%20String%3A%3Afrom%28%22Ada%22%29%2C%0A%20%20%20%20%20%20%20%20idade%3A%2036%2C%0A%20%20%20%20%7D%3B%0A%0A%20%20%20%20let%20nome%20%3D%20p.nome%3B%20%2F%2F%20move%20s%C3%B3%20o%20campo%20%60nome%60%0A%0A%20%20%20%20println%21%28%22nome%20%3D%20%7Bnome%7D%2C%20idade%20%3D%20%7B%7D%22%2C%20p.idade%29%3B%20%2F%2F%20campo%20Copy%2C%20intacto%0A%0A%20%20%20%20%2F%2F%20Nenhuma%20das%20duas%20linhas%20abaixo%20compila%3A%0A%20%20%20%20%2F%2F%0A%20%20%20%20%2F%2F%20%20%20%20%20println%21%28%22%7B%7D%22%2C%20p.nome%29%3B%20%2F%2F%20erro%5BE0382%5D%3A%20borrow%20of%20moved%20value%3A%20%60p.nome%60%0A%20%20%20%20%2F%2F%20%20%20%20%20let%20outra%20%3D%20p%3B%20%20%20%20%20%20%20%20%20%20%2F%2F%20erro%5BE0382%5D%3A%20use%20of%20partially%20moved%20value%3A%20%60p%60%0A%7D%0A
//
// Notebook "move", exemplo 4/5 — "Ways Variables and Data Interact:
// Move", cap. 4.1 do Rust Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// A posse é rastreada campo a campo: mover um campo para fora de uma
// struct é um MOVE PARCIAL. Só aquele campo é invalidado — os outros
// continuam acessíveis, mas a struct já não pode ser usada como um todo.
struct Pessoa {
    nome: String,
    idade: u32,
}

fn main() {
    let p = Pessoa {
        nome: String::from("Ada"),
        idade: 36,
    };

    let nome = p.nome; // move só o campo `nome`

    println!("nome = {nome}, idade = {}", p.idade); // campo Copy, intacto

    // Nenhuma das duas linhas abaixo compila:
    //
    //     println!("{}", p.nome); // erro[E0382]: borrow of moved value: `p.nome`
    //     let outra = p;          // erro[E0382]: use of partially moved value: `p`
}
