"""
Uso do módulo Rust `rust_pyo3_demo`.
Voçe precisa ter rodado `maturin develop` no ambiente virtual ativo para funcionar.
"""

import rust_pyo3_demo as rdemo

print("fibonacci(20) =", rdemo.fibonacci(20))

# função com exceção
print("is_prime(17) =", rdemo.is_prime(17))
try:
    rdemo.is_prime(0)
except ValueError as e:
    print("erro esperado:", e)

# strings/coleções
texto = "Rust é rápido. Python é produtivo. Rust com Python é o melhor dos dois."
print("contagem de palavras:", rdemo.word_count(texto))

# classe com estado
stats = rdemo.RunningStats()
for valor in [10.0, 12.0, 23.0, 23.0, 16.0]:
    stats.push(valor)
print(f"{stats!r} -> média={stats.mean():.2f}, desvio={stats.stddev():.2f}")
