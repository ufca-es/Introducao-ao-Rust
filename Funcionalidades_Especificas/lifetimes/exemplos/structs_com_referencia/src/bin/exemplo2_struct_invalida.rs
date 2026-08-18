// Mesmo `Trecho<'a>` do exemplo 1, mas agora o dado referenciado sai
// de escopo antes da struct ser usada — o compilador rejeita.

struct Trecho<'a> {
    texto: &'a str,
}

fn main() {
    let primeiro;
    {
        let romance = String::from("Era uma vez...");
        primeiro = Trecho { texto: &romance[0..4] };
    } // `romance` é destruída aqui
    println!("{}", primeiro.texto); // erro: `romance` não vive mais
}
