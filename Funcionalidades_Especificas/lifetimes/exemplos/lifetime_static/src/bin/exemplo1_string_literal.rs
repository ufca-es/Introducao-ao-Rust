// String literals são 'static porque ficam gravadas direto no
// binário compilado, não em memória alocada em runtime.

fn string_literal() -> &'static str {
    "isto é um string literal, embutido no binário"
}

fn main() {
    let s: &'static str = string_literal();
    println!("{}", s);
}
