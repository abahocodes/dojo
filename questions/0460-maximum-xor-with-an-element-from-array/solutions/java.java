class Solution {
    private static final int BITS = 30;

    public int[] maximizeXor(int[] nums, int[][] queries) {
        int[] sorted = nums.clone();
        Arrays.sort(sorted);
        Integer[] order = new Integer[queries.length];
        for (int i = 0; i < order.length; i++) order[i] = i;
        Arrays.sort(order, (a, b) -> Integer.compare(queries[a][1], queries[b][1]));

        int[][] child = new int[sorted.length * BITS + 1][2];
        int nodes = 1;
        int[] answer = new int[queries.length];
        Arrays.fill(answer, -1);
        int j = 0;

        for (int qi : order) {
            int x = queries[qi][0], limit = queries[qi][1];
            while (j < sorted.length && sorted[j] <= limit) {
                int v = sorted[j], node = 0;
                for (int b = BITS - 1; b >= 0; b--) {
                    int bit = (v >> b) & 1;
                    if (child[node][bit] == 0) child[node][bit] = nodes++;
                    node = child[node][bit];
                }
                j++;
            }
            if (j == 0) continue;
            int node = 0, best = 0;
            for (int b = BITS - 1; b >= 0; b--) {
                int want = ((x >> b) & 1) ^ 1;
                if (child[node][want] != 0) {
                    best |= 1 << b;
                    node = child[node][want];
                } else {
                    node = child[node][want ^ 1];
                }
            }
            answer[qi] = best;
        }
        return answer;
    }
}
