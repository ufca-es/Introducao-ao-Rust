// Mesmo cenário do exemplo em C (uma função que "cria" e devolve uma
// string), mas em Rust. Aqui não existe free() manual: o sistema de
// ownership (posse) do Rust rastreia, em tempo de compilação, quem é
// o dono da String a cada momento. Quando 'criar_saudacao' retorna
// 'buffer', a POSSE da String é transferida para quem chamou a
// função - a memória so será liberada quando essa nova dona
// ('msg', em 'main') sair de escopo, no fim do programa.
//
// Resultado: nenhum bug de use-after-free é possível aqui, e não
// existe garbage collector rodando em segundo plano.

fn criar_saudacao() -> String {
    let buffer = String::from("Olá, Rust!");
    buffer // a posse de 'buffer' é movida para quem chamou a função
} // nada é liberado aqui, pois a posse já foi transferida, não copiada

fn main() {
    let msg = criar_saudacao();
    println!("{}", msg); // funciona perfeitamente: 'msg' é o dono valido da String
}
