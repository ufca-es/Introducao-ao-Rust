use clap::Parser;

#[derive(Parser)]
#[command(about = "Exemplo de programa de linha de comando em Rust")]
struct Argumentos {
    #[arg(long)]
    nome: String,

    #[arg(long)]
    semestre: u8,
}

fn main() {
    let argumentos = Argumentos::parse();
    println!(
        "Aluno: {} | Semestre: {}",
        argumentos.nome, argumentos.semestre
    );
}
