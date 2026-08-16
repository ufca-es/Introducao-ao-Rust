// Este arquivo esta incorreto de proposito.
// drop recebe a posse do Box, portanto o segundo uso deve ser rejeitado.

fn main() {
    let valor = Box::new(42);

    drop(valor);
    drop(valor);
}
