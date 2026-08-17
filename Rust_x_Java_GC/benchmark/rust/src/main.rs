use std::time::Instant;

/// Estrutura pensada para simular um objeto "de tamanho médio",
/// parecido com o que se encontraria em uma aplicação real
/// (ex: um nó de árvore, um registro de log, um evento de fila).
struct Node {
    value: i64,
    // Só existe para dar ao Node um tamanho realista de alocação; não é
    // lido no benchmark, daí o allow explícito em vez de um warning solto.
    #[allow(dead_code)]
    payload: Vec<i64>,
}

fn allocate_nodes(n: usize) -> Vec<Box<Node>> {
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        nodes.push(Box::new(Node {
            value: i as i64,
            payload: vec![i as i64; 4],
        }));
    }
    nodes
}

fn main() {
    let n: usize = 2_000_000;
    let rounds = 5;

    println!("Rust - alocando {} nos em {} rodadas\n", n, rounds);

    for round in 1..=rounds {
        let start = Instant::now();
        let nodes = allocate_nodes(n);
        let alloc_time = start.elapsed();

        let sum_start = Instant::now();
        let sum: i64 = nodes.iter().map(|node| node.value).sum();
        let sum_time = sum_start.elapsed();

        // Desalocação é determinística: acontece exatamente aqui,
        // não em um momento decidido por um coletor em background.
        let drop_start = Instant::now();
        drop(nodes);
        let drop_time = drop_start.elapsed();

        println!(
            "Rodada {}: alocacao={:>8.2}ms  soma={:>6.2}ms (sum={})  desalocacao={:>6.2}ms  total={:>8.2}ms",
            round,
            alloc_time.as_secs_f64() * 1000.0,
            sum_time.as_secs_f64() * 1000.0,
            sum,
            drop_time.as_secs_f64() * 1000.0,
            (alloc_time + sum_time + drop_time).as_secs_f64() * 1000.0
        );
    }
}
