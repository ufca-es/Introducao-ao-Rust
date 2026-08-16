// Este arquivo é INTENCIONALMENTE quebrado.
// Objetivo: mostrar o erro que o compilador do Rust acusa
// quando tentamos devolver uma referência para algo que
// já foi destruído (dangling reference / use-after-free).
//
// Compile com: rustc dangling_ref.rs
// (vai falhar de propósito - guarde o erro para a apresentação)

fn cria_referencia_pendurada<'a>() -> &'a String {
    let s = String::from("ola, mundo");
    &s
} // `s` sai de escopo AQUI e é destruída (drop automático).
  // A referência que estamos tentando devolver apontaria para
  // memória já liberada.

fn main() {
    let r = cria_referencia_pendurada();
    println!("{}", r);
}
