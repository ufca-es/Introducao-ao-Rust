// 'static não é "desliga a verificação": uma String criada em
// runtime, dentro da função, não pode ser 'static — o dado não vive
// pelo programa inteiro, então o compilador rejeita.

fn nao_e_static() -> &'static str {
    let texto_local = String::from("não dá pra ser 'static");
    &texto_local
}

fn main() {
    println!("{}", nao_e_static());
}
