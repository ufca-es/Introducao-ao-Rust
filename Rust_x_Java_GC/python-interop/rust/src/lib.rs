use pyo3::prelude::*;

/// Função "pesada" o suficiente para deixar a diferença de desempenho
/// visível quando chamada a partir do Python.
#[pyfunction]
fn soma_quadrados(n: i64) -> i128 {
    // i128 evita overflow: a soma de quadrados cresce com n^3,
    // e para n na casa de dezenas de milhoes isso ultrapassa
    // facilmente o limite de um i64 (~9.2 * 10^18).
    (0..n).map(|x| (x as i128) * (x as i128)).sum()
}

/// Ponto de entrada do módulo Python. O nome aqui precisa bater com
/// `name` em [lib] no Cargo.toml.
#[pymodule]
fn rust_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(soma_quadrados, m)?)?;
    Ok(())
}
