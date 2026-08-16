import java.util.ArrayList;
import java.util.List;

public class Benchmark {

    // Mesma "forma" de objeto do lado Rust, para a comparação ser justa.
    static class Node {
        long value;
        long[] payload;

        Node(long value) {
            this.value = value;
            this.payload = new long[]{value, value, value, value};
        }
    }

    static List<Node> allocateNodes(int n) {
        List<Node> nodes = new ArrayList<>(n);
        for (int i = 0; i < n; i++) {
            nodes.add(new Node(i));
        }
        return nodes;
    }

    public static void main(String[] args) {
        int n = 2_000_000;
        int rounds = 5;

        System.out.printf("Java - alocando %d nos em %d rodadas%n%n", n, rounds);

        for (int round = 1; round <= rounds; round++) {
            long allocStart = System.nanoTime();
            List<Node> nodes = allocateNodes(n);
            long allocTime = System.nanoTime() - allocStart;

            long sumStart = System.nanoTime();
            long sum = 0;
            for (Node node : nodes) {
                sum += node.value;
            }
            long sumTime = System.nanoTime() - sumStart;

            // Aqui NÃO chamamos System.gc() manualmente: a ideia é deixar o
            // coletor decidir sozinho quando agir, exatamente como acontece
            // numa aplicação real. As pausas ficam visíveis no log de GC
            // (veja as instruções no README) e não nesta medição isolada.
            nodes = null;

            System.out.printf(
                "Rodada %d: alocacao=%8.2fms  soma=%6.2fms (sum=%d)  total=%8.2fms%n",
                round,
                allocTime / 1e6, sumTime / 1e6, sum,
                (allocTime + sumTime) / 1e6
            );
        }
    }
}
