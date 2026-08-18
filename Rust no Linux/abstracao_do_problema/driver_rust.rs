// Este bloco e o binding: ele descreve para o Rust a funcao existente em C.
unsafe extern "C" {
    fn dma_transferir_v2(dados: *const i32, quantidade: usize) -> i32;
}

// Esta funcao e uma abstracao segura mantida pelo lado Rust.
// O restante do programa nao precisa lidar diretamente com ponteiros.
fn transferir_com_seguranca(dados: &[i32]) -> Result<(), &'static str> {
    if dados.is_empty() {
        return Err("a lista de dados esta vazia");
    }

    // A chamada e unsafe porque o compilador Rust nao consegue verificar o C.
    let resultado = unsafe { dma_transferir_v2(dados.as_ptr(), dados.len()) };

    if resultado == 0 {
        Ok(())
    } else {
        Err("a camada C recusou a transferencia")
    }
}

fn main() {
    let dados = [10, 20, 30, 40];

    println!("[Driver Rust] Chamando a API C por meio do binding...");
    match transferir_com_seguranca(&dados) {
        Ok(()) => println!("[Driver Rust] Tudo certo."),
        Err(erro) => eprintln!("[Driver Rust] Erro: {erro}"),
    }
}
