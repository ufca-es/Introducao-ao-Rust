# Rust x Linguagem com Garbage Collector (Java)

Material de apoio para o subtema **"Rust x Linguagem com Garbage Collector (Java)"**,
parte do trabalho de **Introdução ao Rust: Segurança de Memória sem Garbage Collector**
(Disciplina de Paradigmas de Programação — UFCA).

Todo o código deste repositório foi **compilado e executado de verdade** —
verificado de forma independente em uma máquina Windows com Rust 1.97.1 (MSVC),
JDK 21 (Eclipse Temurin) e Python 3.14, e os números abaixo são as saídas reais
capturadas nessa verificação, não estimativas.

Ainda assim, recomendamos rodar tudo de novo na máquina de vocês antes da
entrega final: os números absolutos variam bastante de máquina para máquina —
inclusive o *coletor de GC* que a JVM escolhe automaticamente muda conforme o
número de núcleos e a memória disponível (nesta verificação, por exemplo, a
JVM escolheu G1 em vez de Serial GC, o que mudou o formato das pausas — ver
seção 2). O que **não muda** é o padrão qualitativo: Rust com desempenho
estável e previsível, Java sujeito à variância introduzida pelo GC.

## Estrutura da pasta

```
Rust_x_Java_GC/
├── benchmark/            # desempenho e footprint de memória
│   ├── rust/
│   └── java/
├── gc-comparison/         # classes de erro evitadas (ou não) em tempo de compilação
│   ├── rust/
│   └── java/
└── python-interop/        # interoperabilidade com Python (requisito do trabalho)
    ├── rust/               # via PyO3
    └── java/               # via Py4J
```

---

## 1. Fundamentação teórica (resumo para a apresentação)

**Java (JVM):** o heap é dividido em gerações (young/old). Objetos "nascem" na
young generation; um coletor (G1 é o padrão desde o Java 9, ZGC e Shenandoah
são alternativas de baixa pausa) periodicamente identifica objetos sem
referências vivas e os libera. Isso é automático, mas introduz **pausas** —
momentos em que threads da aplicação são suspensas ("stop-the-world") — cuja
frequência e duração dependem da taxa de alocação e do tamanho do heap.

**Rust:** não existe coletor. Cada valor tem um único "dono" (*ownership*).
Quando o dono sai de escopo, o compilador insere automaticamente a chamada que
libera a memória (isso é o mecanismo RAII). O *borrow checker* garante em
tempo de compilação que não existam referências penduradas (*dangling*) nem
acesso concorrente inseguro — sem custo nenhum em tempo de execução.

A tese central da comparação: **GC troca uma classe de erros de memória por
previsibilidade de desempenho** — e Rust faz a troca oposta.

---

## 2. Benchmark de alocação e desempenho (`benchmark/`)

Ambos os programas fazem a mesma coisa: alocam 2 milhões de objetos pequenos
(um `Node` com um inteiro e um payload de 4 posições), somam um campo de todos
eles, e então liberam tudo. 5 rodadas seguidas, no mesmo processo.

### Rust — execução real

```
Rust - alocando 2000000 nos em 5 rodadas

Rodada 1: alocacao=  236.97ms  soma= 10.06ms (sum=1999999000000)  desalocacao=138.61ms  total=  385.65ms
Rodada 2: alocacao=  231.75ms  soma= 10.36ms (sum=1999999000000)  desalocacao=140.95ms  total=  383.06ms
Rodada 3: alocacao=  232.01ms  soma= 10.59ms (sum=1999999000000)  desalocacao=148.06ms  total=  390.66ms
Rodada 4: alocacao=  232.86ms  soma= 10.95ms (sum=1999999000000)  desalocacao=141.27ms  total=  385.08ms
Rodada 5: alocacao=  234.27ms  soma= 10.29ms (sum=1999999000000)  desalocacao=140.88ms  total=  385.44ms
```

Repare como **todas as rodadas** ficam dentro de uma faixa estreita
(383–391ms), rodada 1 incluída — nada de aquecimento surpresa nem picos.
A desalocação é sempre uma fração previsível do total (~140ms), porque
acontece de forma determinística no exato ponto em que os nós saem de escopo,
não em um momento decidido por um coletor em background.

### Java — execução real, com log de GC habilitado

```
Java - alocando 2000000 nos em 5 rodadas

Rodada 1: alocacao=   88.97ms  soma= 17.72ms (sum=1999999000000)  total=  106.69ms
Rodada 2: alocacao=  106.29ms  soma= 28.22ms (sum=1999999000000)  total=  134.51ms
Rodada 3: alocacao=   84.49ms  soma= 11.92ms (sum=1999999000000)  total=   96.41ms
Rodada 4: alocacao=   94.02ms  soma= 14.64ms (sum=1999999000000)  total=  108.66ms
Rodada 5: alocacao=   66.70ms  soma= 12.45ms (sum=1999999000000)  total=   79.15ms
```

Nesta máquina o alocador do Java (que faz *bump allocation* na young
generation) chega a ser **mais rápido por rodada** que o Rust — o que parece
contraintuitivo até lembrar que aqui a alocação não inclui o custo de
desalocação (o Rust soma os dois). O ponto não é "quem aloca mais rápido", e
sim **onde mora o custo**: no Rust ele é pago de forma explícita e
determinística logo depois da alocação; no Java ele é adiado e pago mais
tarde, de uma vez, pelo coletor — como mostra o log de GC real (arquivo
completo em `benchmark/java/gc_exemplo.log`):

```
[info][gc] Using G1
GC(0)  Pause Young (Normal) (G1 Evacuation Pause) 29M->28M(256M) 10.000ms
GC(5)  Pause Young (Concurrent Start) (G1 Humongous Allocation) 152M->154M(359M) 10.507ms
GC(9)  Concurrent Mark Cycle
GC(13) Pause Young (Prepare Mixed) (G1 Evacuation Pause) 295M->297M(421M) 15.416ms
GC(14) Pause Young (Mixed) (G1 Evacuation Pause) 343M->341M(421M) 11.231ms
```

Aqui a JVM escolheu automaticamente o **G1** (não o Serial GC), porque a
máquina tem núcleos e memória suficientes para isso — e o resultado são
pausas curtas e frequentes (6–21ms), em vez de poucas pausas longas.

Para mostrar o outro extremo, rodamos de novo forçando `-XX:+UseSerialGC`
(arquivo completo em `benchmark/java/gc_serial.log`):

```
[info][gc] Using Serial
GC(0) Pause Young (Allocation Failure)  68M->62M(247M)  54.840ms
GC(1) Pause Young (Allocation Failure) 130M->130M(247M) 71.102ms
GC(2) Pause Young (Allocation Failure) 199M->199M(267M) 71.671ms
GC(3) Pause Full  (Allocation Failure) 199M->54M(247M)  89.822ms
```

Com Serial, as pausas sobem para **55–90ms** — 3 a 10x maiores que as pausas
do G1 na mesma máquina, mesmo processando a mesma carga. Isso confirma o
ponto qualitativo (coletor mais simples = pausas maiores), mas **não**
reproduz o pico de 654ms que aparecia numa versão anterior deste material
gerada em outro ambiente (provavelmente um container com poucos núcleos e
memória bem mais restrita, o que faz o `Full GC` demorar bem mais). É um bom
exemplo prático de por que benchmarks de GC **precisam** citar o hardware e
os flags usados — o mesmo código, no mesmo dia, dá números bem diferentes
dependendo de quantos núcleos e quanta memória a JVM enxerga. O ponto que se
mantém em qualquer coletor e qualquer máquina: o Rust não tem **nenhuma**
dessas pausas, porque não existe um processo em background decidindo quando
liberar memória.

### Como reproduzir

```bash
# Rust
cd benchmark/rust
cargo build --release
./target/release/bench_rust

# Java (com log de GC, coletor padrão da JVM na sua máquina)
cd benchmark/java
javac Benchmark.java
java -Xlog:gc:file=gc.log:time,uptime,level,tags -Xmx512m Benchmark
cat gc.log

# Java forçando o Serial GC, para comparar o tamanho das pausas
java -XX:+UseSerialGC -Xlog:gc:file=gc_serial.log:time,uptime,level,tags -Xmx512m Benchmark
cat gc_serial.log
```

> No Windows, se você usar Git Bash/MSYS, tome cuidado: o `link.exe` da MSVC
> (necessário para compilar o lado Rust) pode ser sombreado pelo `link.exe` de
> coreutils do Git. Se o build do Rust falhar com um erro estranho de "extra
> operand" vindo do `link`, rode o `cargo build` a partir de um **Developer
> PowerShell/Command Prompt for VS** em vez do Git Bash.

---

## 3. Erro evitado em tempo de compilação (`gc-comparison/`)

### Rust: o compilador recusa a referência pendurada

`gc-comparison/rust/dangling_ref.rs` tenta devolver uma referência para uma
`String` que é destruída ao final da função — um *use-after-free* clássico em
C/C++. Em Rust, isso **nem chega a compilar**:

```
$ rustc dangling_ref.rs
error[E0515]: cannot return reference to local variable `s`
  --> dangling_ref.rs:11:5
   |
11 |     &s
   |     ^^ returns a reference to data owned by the current function
```

A versão corrigida (`fixed.rs`) resolve isso devolvendo a **posse** (ownership)
do valor em vez de uma referência — e compila e roda normalmente.

### Java: o mesmo tipo de bug simplesmente não existe... mas outro aparece

Em Java não dá pra escrever o equivalente ao `dangling_ref.rs`: o GC nunca
libera um objeto enquanto alguém ainda o referencia, então não existe
ponteiro para memória já desalocada. **Esse é justamente o ponto da
comparação** — só que a moeda tem dois lados.

`gc-comparison/java/LeakyCache.java` simula um erro de projeto extremamente
comum: uma "cache" estática que nunca é limpa. O código **compila sem nenhum
aviso** e roda perfeitamente — até estourar a memória:

```
Requisicao 0  - memoria usada:   4 MB - cache.size()=1
Requisicao 20 - memoria usada:  44 MB - cache.size()=21
Requisicao 40 - memoria usada:  84 MB - cache.size()=41
Requisicao 60 - memoria usada: 122 MB - cache.size()=61
Requisicao 80 - memoria usada: 162 MB - cache.size()=81
Exception in thread "main" java.lang.OutOfMemoryError: Java heap space
	at LeakyCache.handleRequest(LeakyCache.java:31)
	at LeakyCache.main(LeakyCache.java:38)
```

(execução real com `java -Xmx200m LeakyCache`; o número exato de requisições
até o `OutOfMemoryError` varia por máquina, mas o comportamento — crescimento
linear até estourar — é sempre o mesmo)

O compilador Java não tem como saber que aquela referência na lista estática
era "para ser temporária" — do ponto de vista dele, o código está correto.
**O GC previne use-after-free, mas não previne vazamentos lógicos causados
por referências esquecidas.**

> **Contraponto para a apresentação (importante para não soar propaganda de
> Rust):** o Rust também pode vazar memória, por exemplo com ciclos de
> referência usando `Rc<RefCell<T>>` — dois valores que se referenciam
> mutuamente nunca chegam a contagem de referências zero. O *ownership* evita
> use-after-free e data races, mas não é uma garantia absoluta contra todo
> tipo de leak lógico. Vale citar isso para mostrar domínio crítico do tema.

### Como reproduzir

```bash
# Rust - vai falhar de propósito
cd gc-comparison/rust
rustc dangling_ref.rs   # mostra o erro E0515
rustc fixed.rs && ./fixed   # versão corrigida, funciona

# Java - compila e roda até estourar memória
cd gc-comparison/java
javac LeakyCache.java
java -Xmx200m LeakyCache
```

---

## 4. Interoperabilidade com Python (`python-interop/`)

Requisito obrigatório do trabalho. Aqui a comparação é sobre **como** cada
linguagem se conecta ao Python, e qual o custo disso.

### Rust via PyO3 — roda dentro do processo Python

Rust vira uma biblioteca compartilhada (`.so`) carregada diretamente pelo
interpretador Python, sem processo nem runtime extra.

```python
# python-interop/rust/benchmark_python.py — execução real
Comparando soma de quadrados ate n=100000000

Rust (via PyO3): resultado=333333328333333350000000  tempo=0.008ms
Python puro:     resultado=333333328333333350000000  tempo=10719.340ms

Rust foi 1392122.0x mais rapido que o loop puro em Python
```

A diferença é enorme porque está comparando um laço interpretado, elemento
por elemento, contra código de máquina compilado e vetorizado — é exatamente
o cenário em que a interoperabilidade Rust↔Python compensa mais: usar Python
para orquestração e Rust para o "loop quente".

> Ao apresentar, vale deixar claro que esse número gigante (~1,4 milhão de
> vezes) é um caso favorável ao extremo (laço puro em Python é o pior caso
> possível). Uma comparação mais justa incluiria também uma versão com
> NumPy vetorizado, que fica entre os dois extremos — fica como sugestão de
> aprofundamento se sobrar tempo.

**Como reproduzir:**
```bash
cd python-interop/rust
python -m venv .venv
# Linux/Mac:
source .venv/bin/activate
# Windows:
.venv\Scripts\activate

pip install maturin
maturin develop --release
python benchmark_python.py
```

> **Nota de compatibilidade descoberta ao testar:** o `Cargo.toml` original
> pedia `pyo3 = "0.22"`, que **não compila** em Python 3.14 (lançado depois
> dessa versão do PyO3) — o build falha com "the configured Python
> interpreter version (3.14) is newer than PyO3's maximum supported version
> (3.13)". Atualizamos para `pyo3 = "0.29"`, que já suporta Python 3.14. Se no
> PC de vocês o `maturin develop` reclamar de versão do Python, é esse o
> primeiro lugar a olhar — confiram a versão do PyO3 contra a versão do Python
> instalada. Também é preciso ter um virtualenv ativo (`.venv`) para o
> `maturin develop` funcionar; sem isso ele recusa rodar.

### Java via Py4J — precisa de uma JVM rodando à parte

Diferente do PyO3, o Py4J não carrega nada "dentro" do processo Python: ele
sobe uma JVM separada e conversa com ela por socket TCP local. Isso tem um
custo de inicialização que o Rust simplesmente não tem:

```
Java (via Py4J): resultado=2666664666667000000  tempo_da_chamada=55.716ms   (1a chamada)
Java (via Py4J): resultado=2666664666667000000  tempo_da_chamada= 2.411ms   (2a chamada)
Java (via Py4J): resultado=2666664666667000000  tempo_da_chamada= 2.069ms   (3a chamada)
```

A primeira chamada é sempre a mais lenta — paga o custo de estabelecer a
conexão TCP e "aquecer" a serialização; chamadas seguintes na mesma conexão
caem para poucos milissegundos. Em uma das execuções, a primeira chamada
chegou a levar **4,5 segundos** (provavelmente carregamento de classes Java a
frio, ou antivírus escaneando o processo recém-criado) — o que reforça, na
prática, o ponto teórico: cada chamada via Py4J passa por serialização e
socket, então está sujeita a variância que uma chamada de função em memória
(como a do PyO3) simplesmente não tem.

**Como reproduzir (Linux/Mac):**
```bash
cd python-interop/java
pip install py4j
JAR=$(python3 -c "import py4j,os;print(os.path.dirname(py4j.__file__)+'/../../../share/py4j')")/py4j*.jar
javac -cp $(ls $JAR) SomaQuadradosEntryPoint.java
java -cp .:$(ls $JAR) SomaQuadradosEntryPoint &   # deixa rodando em background
python3 cliente_py4j.py
```

**Como reproduzir (Windows / PowerShell):** o classpath do Java usa `;` em
vez de `:`, e o caminho do `.jar` instalado pelo pip fica em
`<python>\share\py4j\py4j<versão>.jar` (confirme com
`python -c "import py4j, os; print(os.path.dirname(py4j.__file__))"` e suba
um nível até `share`):
```powershell
cd python-interop/java
pip install py4j
javac -cp "<caminho>\py4jX.Y.Z.jar" SomaQuadradosEntryPoint.java
java -cp ".;<caminho>\py4jX.Y.Z.jar" SomaQuadradosEntryPoint   # roda em outra janela/aba
python cliente_py4j.py
```

---

## 5. Roteiro sugerido para a apresentação (12–15 min)

1. **Motivação** (2 min) — por que gerenciamento de memória manual em C/C++
   causava tantos bugs, e a resposta de duas gerações diferentes de
   linguagens: GC (anos 90, Java) vs *ownership* em tempo de compilação
   (anos 2010, Rust).
2. **Conceitos** (3 min) — ownership/borrowing/lifetimes vs gerações do GC.
3. **Demo ao vivo do erro de compilação** (2 min) — rodar `rustc
   dangling_ref.rs` na tela e mostrar o erro E0515; comparar com o
   `LeakyCache.java` compilando "normalmente".
4. **Benchmark** (3 min) — mostrar os números de latência/variância, e se
   possível o gráfico das pausas do `gc.log`.
5. **Interoperabilidade com Python** (2 min) — PyO3 vs Py4J, e por que isso
   importa na prática (ex: acelerar um trecho quente de um pipeline Python).
6. **Análise crítica e conclusão** (2–3 min) — quando cada abordagem faz mais
   sentido (ver tabela abaixo).

| Cenário | Melhor opção | Por quê |
|---|---|---|
| Sistemas real-time / embarcados / trading de baixa latência | Rust | Sem pausas de GC, latência previsível |
| Aplicação corporativa, equipe grande, prazo curto | Java | Produtividade, curva de aprendizado menor, ecossistema maduro |
| Biblioteca de alto desempenho chamada a partir de Python | Rust (PyO3) | Sem overhead de runtime/processo separado |
| Backend com integração via processo externo já existente | Java (Py4J/gRPC) | Ecossistema JVM maduro para esse tipo de integração |

---

## 6. Referências

- The Rust Programming Language — capítulo sobre Ownership:
  https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
- The Rust Programming Language — Lifetimes:
  https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
- PyO3 User Guide: https://pyo3.rs/
- Py4J — documentação oficial: https://www.py4j.org/
- Oracle — HotSpot Virtual Machine Garbage Collection Tuning Guide (Java 21):
  https://docs.oracle.com/en/java/javase/21/gctuning/introduction-garbage-collection-tuning.html
- Especificação do trabalho prático — UFCA, disciplina de Paradigmas de
  Programação (documento-base fornecido pelo professor).

*(Adicionem aqui os demais materiais que consultarem durante a pesquisa —
o enunciado exige uma seção de referências bibliográficas completa no
repositório final.)*
