// Exemplo propositalmente incorreto.
// A antiga API se chamava "dma_transferir", mas a equipe C a mudou para
// "dma_transferir_v2". Este binding ainda nao acompanhou a mudanca.
unsafe extern "C" {
    fn dma_transferir(dados: *const i32, quantidade: usize) -> i32;
}

fn main() {
    let dados = [10, 20, 30, 40];
    let resultado = unsafe { dma_transferir(dados.as_ptr(), dados.len()) };
    println!("Resultado: {resultado}");
}
