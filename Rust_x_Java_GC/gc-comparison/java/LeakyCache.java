import java.util.ArrayList;
import java.util.List;

/**
 * Este código COMPILA e RODA sem erro nenhum em Java - e esse é
 * justamente o ponto da comparação.
 *
 * Em Java, o equivalente ao "dangling_ref.rs" simplesmente não existe:
 * o GC nunca libera um objeto enquanto ainda houver referência viva
 * para ele, então não há como ter um ponteiro para memória já
 * desalocada (use-after-free).
 *
 * O preço disso é o oposto: como o GC só limpa o que NINGUÉM mais
 * referencia, é fácil vazar memória "esquecendo" uma referência viva
 * em algum lugar (aqui, uma lista estática que nunca é limpa).
 * O programa vai crescer em uso de memória até estourar
 * OutOfMemoryError - e o compilador não vai acusar nada, porque do
 * ponto de vista do Java o código está perfeitamente correto.
 *
 * Compile com: javac LeakyCache.java
 * Rode com:    java -Xmx256m LeakyCache
 */
public class LeakyCache {

    // "Cache" que devia ter um limite de tamanho ou uma política de
    // expiração, mas foi esquecida - um erro de projeto extremamente
    // comum em aplicações Java reais.
    private static final List<byte[]> cache = new ArrayList<>();

    static void handleRequest(int requestId) {
        byte[] response = new byte[1024 * 1024]; // 1 MB "de resposta"
        cache.add(response); // referência nunca é removida
    }

    public static void main(String[] args) {
        Runtime rt = Runtime.getRuntime();
        for (int i = 0; i < 100000; i++) {
            handleRequest(i);
            if (i % 20 == 0) {
                long usedMB = (rt.totalMemory() - rt.freeMemory()) / (1024 * 1024);
                System.out.println("Requisicao " + i + " - memoria usada: " + usedMB + " MB - cache.size()=" + cache.size());
            }
        }
    }
}
