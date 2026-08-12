use serde::Serialize;

#[derive(Serialize)]
struct Aluno {
    nome: String,
    semestre: u8,
}

fn main() {
    let aluno = Aluno {
        nome: String::from("Neto"),
        semestre: 4,
    };

    let json = serde_json::to_string_pretty(&aluno).expect("falha ao gerar JSON");
    println!("{json}");
}
