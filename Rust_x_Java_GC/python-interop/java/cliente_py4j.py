import time
from py4j.java_gateway import JavaGateway

gateway = JavaGateway()
app = gateway.entry_point

# N menor que no exemplo Rust: em Java a soma é feita em `long` (64 bits,
# sem verificação de overflow por padrão), e n=100_000_000 estouraria
# esse limite silenciosamente (o resultado ficaria errado sem nenhum
# aviso). N=2_000_000 fica com folga segura dentro do limite do long.
N = 2_000_000

start = time.perf_counter()
resultado = app.somaQuadrados(N)
elapsed = time.perf_counter() - start

print(f"Java (via Py4J): resultado={resultado}  tempo_da_chamada={elapsed * 1000:.3f}ms")
print("(este tempo NAO inclui subir a JVM do gateway, que já precisa estar rodando antes)")

gateway.shutdown()
