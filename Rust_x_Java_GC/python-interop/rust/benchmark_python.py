import time
import rust_lib


def soma_quadrados_python(n: int) -> int:
    return sum(x * x for x in range(n))


N = 100_000_000

print(f"Comparando soma de quadrados ate n={N}\n")

start = time.perf_counter()
resultado_rust = rust_lib.soma_quadrados(N)
tempo_rust = time.perf_counter() - start
print(f"Rust (via PyO3): resultado={resultado_rust}  tempo={tempo_rust * 1000:.3f}ms")

start = time.perf_counter()
resultado_python = soma_quadrados_python(N)
tempo_python = time.perf_counter() - start
print(f"Python puro:     resultado={resultado_python}  tempo={tempo_python * 1000:.3f}ms")

assert resultado_rust == resultado_python, "os resultados deveriam ser identicos!"
print(f"\nRust foi {tempo_python / tempo_rust:.1f}x mais rapido que o loop puro em Python")
