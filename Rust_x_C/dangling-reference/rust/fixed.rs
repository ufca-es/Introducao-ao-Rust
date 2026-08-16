fn criar_valor() -> Box<i32> {
    Box::new(42)
}

fn main() {
    let valor = criar_valor();
    println!("Valor: {valor}");
}
