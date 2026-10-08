class Solution {
    public int findMaximizedCapital(int k, int w, int[] profits, int[] capital) {
        int n = profits.length;
        Integer[] order = new Integer[n];
        for (int i = 0; i < n; i++) order[i] = i;
        Arrays.sort(order, (a, b) -> Integer.compare(capital[a], capital[b]));
        PriorityQueue<Integer> affordable = new PriorityQueue<>(Collections.reverseOrder());
        int p = 0;
        for (int round = 0; round < k; round++) {
            while (p < n && capital[order[p]] <= w) affordable.add(profits[order[p++]]);
            if (affordable.isEmpty()) break;
            w += affordable.poll();
        }
        return w;
    }
}
