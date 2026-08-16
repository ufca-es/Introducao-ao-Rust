import py4j.GatewayServer;

/**
 * Expõe um método Java para ser chamado a partir do Python via Py4J.
 * Diferente do PyO3 (Rust), aqui não existe um binário nativo carregado
 * diretamente no processo Python: o Py4J sobe uma JVM separada e
 * conversa com ela por socket TCP. É esse overhead de processo/JVM
 * que a gente quer evidenciar na comparação com o Rust.
 */
public class SomaQuadradosEntryPoint {

    public long somaQuadrados(long n) {
        long total = 0;
        for (long x = 0; x < n; x++) {
            total += x * x;
        }
        return total;
    }

    public static void main(String[] args) {
        GatewayServer server = new GatewayServer(new SomaQuadradosEntryPoint());
        server.start();
        System.out.println("Gateway Py4J rodando...");
    }
}
