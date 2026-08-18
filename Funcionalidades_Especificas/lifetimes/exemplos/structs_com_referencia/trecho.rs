// Exemplo: struct com referência (lifetime obrigatório na declaração)
// Uma instância de `Trecho` nunca pode sobreviver ao `String` que ela
// referencia — o compilador garante isso via `'a`.

struct Trecho<'a> {
    texto: &'a str,
}

fn main() {
    let romance = String::from("Era uma vez...");
    let primeiro = Trecho { texto: &romance[0..4] };
    println!("{}", primeiro.texto); // válido: `romance` ainda vive aqui
}

/*
Se `romance` saísse de escopo antes de `primeiro` ser usado, o erro seria:

error[E0597]: `romance` does not live long enough
  |
  | let primeiro = Trecho { texto: &romance[0..4] };
  |                                 ^^^^^^^^^^^^^^ borrowed value does not
  |                                                live long enough
*/
