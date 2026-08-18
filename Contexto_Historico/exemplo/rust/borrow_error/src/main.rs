// Este arquivo demonstra o "borrow checker" do Rust em ação.
// Tentamos guardar uma referência (&x) a uma variável que sai de
// escopo antes da referência ser usada - o equivalente conceitual
// do "dangling pointer" do exemplo em C (use_after_free.c).
//
// PROPOSITALMENTE, este código NÃO COMPILA. Rode 'cargo build' (ou
// 'cargo run') para ver o compilador recusar o programa, evitando
// que o bug de memória sequer chegue a gerar um binário.

fn main() {
    let r;
    {
        let x = String::from("dado temporario");
        r = &x; // tentamos guardar uma referência a 'x'
    } // 'x' sai de escopo aqui e sua memória é liberada automaticamente

    println!("{}", r); // ERRO DE COMPILAÇÃO: 'x' nao vive o suficiente
}
