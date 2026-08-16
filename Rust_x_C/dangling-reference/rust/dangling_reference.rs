// Este arquivo esta incorreto de proposito.
// O compilador impede que a referencia sobreviva ao valor local.

fn criar_referencia_pendurada<'a>() -> &'a i32 {
    let valor = 42;
    &valor
}

fn main() {
    let referencia = criar_referencia_pendurada();
    println!("Valor: {referencia}");
}
