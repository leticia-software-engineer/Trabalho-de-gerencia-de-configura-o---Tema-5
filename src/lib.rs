//! Funções e classe em Rust expostas ao Python via PyO3.
//! `#[pyfunction]` -> vira função Python. `#[pyclass]` -> vira classe Python.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

// Fibonacci.
// entre Rust e Python (u64 <-> int).
#[pyfunction]
fn fibonacci(n: u64) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        let tmp = a;
        a = b;
        b = tmp + b;
    }
    a
}

// Testa se n é primo. Retorna PyResult para poder lançar exceção Python
// (Err vira ValueError do lado do Python).
#[pyfunction]
fn is_prime(n: u64) -> PyResult<bool> {
    if n == 0 {
        return Err(PyValueError::new_err("n deve ser maior que 0"));
    }
    if n == 1 {
        return Ok(false);
    }
    let mut i = 2u64;
    while i * i <= n {
        if n % i == 0 {
            return Ok(false);
        }
        i += 1;
    }
    Ok(true)
}

// Conta ocorrências de cada palavra em um texto. HashMap<String, u64> vira
// dict do lado Python automaticamente.
#[pyfunction]
fn word_count(text: &str) -> HashMap<String, u64> {
    let mut counts: HashMap<String, u64> = HashMap::new();
    for raw_word in text.split_whitespace() {
        let word: String = raw_word
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();
        if !word.is_empty() {
            *counts.entry(word).or_insert(0) += 1;
        }
    }
    counts
}

// Acumulador estatístico, exposto como classe Python.
#[pyclass]
struct RunningStats {
    count: u64,
    sum: f64,
    sum_sq: f64,
}

#[pymethods]
impl RunningStats {
    #[new]
    fn new() -> Self {
        RunningStats {
            count: 0,
            sum: 0.0,
            sum_sq: 0.0,
        }
    }

    fn push(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;
        self.sum_sq += value * value;
    }

    fn mean(&self) -> PyResult<f64> {
        if self.count == 0 {
            return Err(PyValueError::new_err("nenhum valor adicionado ainda"));
        }
        Ok(self.sum / self.count as f64)
    }

    fn stddev(&self) -> PyResult<f64> {
        if self.count == 0 {
            return Err(PyValueError::new_err("nenhum valor adicionado ainda"));
        }
        let mean = self.sum / self.count as f64;
        let variance = (self.sum_sq / self.count as f64) - (mean * mean);
        Ok(variance.max(0.0).sqrt())
    }

    fn __repr__(&self) -> String {
        format!("RunningStats(count={})", self.count)
    }
}

// Ponto de entrada de módulo: define o que fica visível em `import rust_pyo3_demo`.
#[pymodule]
fn rust_pyo3_demo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(is_prime, m)?)?;
    m.add_function(wrap_pyfunction!(word_count, m)?)?;
    m.add_class::<RunningStats>()?;
    Ok(())
}

// Testes do Rust, rodam com `cargo test`. Não precisa do Python.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(10), 55);
    }
}
