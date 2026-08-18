// Exemplo: anotação explícita de lifetime
// Quando a elision não resolve sozinha (mais de um parâmetro de
// referência), a anotação `'a` é obrigatória.

fn maior<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let resultado;
    {
        let string2 = String::from("xyz");
        resultado = maior(string1.as_str(), string2.as_str());
        println!("A maior string é {}", resultado);
    }
    // Se tentássemos usar `resultado` aqui fora, o compilador rejeitaria:
    // `string2` não vive mais, e `'a` amarra o retorno ao menor dos dois
    // lifetimes de entrada.
}

/*
Sem a anotação `<'a>`, o compilador rejeita com:

error[E0106]: missing lifetime specifier
  |
  | fn maior(x: &str, y: &str) -> &str {
  |             ----     ----     ^ expected named lifetime parameter
  = help: this function's return type contains a borrowed value, but the
    signature does not say whether it is borrowed from `x` or `y`
*/
