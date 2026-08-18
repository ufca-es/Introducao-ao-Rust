// Exemplo: lifetime 'static
// `'static` é apenas o lifetime mais longo possível (dura o programa
// inteiro) — não é um "desliga o borrow checker".

fn string_literal() -> &'static str {
    "isto é um string literal, embutido no binário" // válido: vive por
                                                      // toda a execução
}

fn main() {
    let s: &'static str = string_literal();
    println!("{}", s);

    // Contraste: isto NÃO seria 'static, mesmo com a anotação, porque
    // `texto_local` é criado em tempo de execução dentro de `main` e
    // não vive além do escopo de `main`:
    //
    // fn nao_e_static() -> &'static str {
    //     let texto_local = String::from("não dá pra ser 'static");
    //     &texto_local // erro: `texto_local` não vive por 'static
    // }
}
