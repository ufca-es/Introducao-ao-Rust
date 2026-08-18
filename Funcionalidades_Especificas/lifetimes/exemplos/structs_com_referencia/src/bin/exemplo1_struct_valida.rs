// Uma struct que guarda uma referência precisa declarar o lifetime
// dela — o compilador garante que a struct não sobrevive ao dado
// que ela referencia.

struct Trecho<'a> {
    texto: &'a str,
}

fn main() {
    let romance = String::from("Era uma vez...");
    let primeiro = Trecho { texto: &romance[0..4] };
    println!("{}", primeiro.texto); // válido: `romance` ainda vive aqui
}
