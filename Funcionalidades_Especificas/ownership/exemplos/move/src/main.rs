// Baseado em "Ways Variables and Data Interact: Move" — cap. 4.1 do Rust
// Book:
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // move: a posse do buffer no heap passa de s1 para s2

    println!("s2 = {s2}");

    // A linha abaixo NÃO compila. Descomente para ver o erro:
    //
    //     println!("s1 = {s1}");
    //
    // erro[E0382]: borrow of moved value: `s1`
    //   move occurs because `s1` has type `String`, which does not
    //   implement the `Copy` trait
    //
    // Internamente, `s1` e `s2` apontavam para o MESMO endereço no heap
    // (uma atribuição só copia ponteiro, tamanho e capacidade, nunca o
    // conteúdo do heap). Se os dois nomes continuassem válidos, ao saírem
    // de escopo o Rust chamaria `drop` duas vezes sobre o mesmo endereço
    // — um double free. Em vez de deixar isso acontecer em tempo de
    // execução, o compilador invalida `s1` no momento do move.

    // Reatribuir uma variável também dispara `drop` imediato do valor
    // antigo, antes mesmo do fim do escopo:
    let mut s3 = String::from("hello");
    println!("s3 antes = {s3}");
    s3 = String::from("ahoy"); // o "hello" antigo é liberado agora
    println!("s3 depois = {s3}");
}
