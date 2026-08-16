// Versão corrigida de dangling_ref.rs.
// Em vez de devolver uma REFERÊNCIA para `s` (que seria destruída
// ao sair da função), devolvemos a posse (ownership) do próprio
// valor. Quem chama a função passa a ser dono da String.
//
// Compile com: rustc fixed.rs && ./fixed

fn cria_string() -> String {
    let s = String::from("ola, mundo");
    s // move: a posse de `s` é transferida para quem chamou
}

fn main() {
    let r = cria_string();
    println!("{}", r);
}
