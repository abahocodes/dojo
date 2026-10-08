class Solution {
    public double mincostToHireWorkers(int[] quality, int[] wage, int k) {
        int n = quality.length;
        // Sort workers by wage/quality ratio, compared exactly by cross-multiplying.
        Integer[] order = new Integer[n];
        for (int i = 0; i < n; i++) order[i] = i;
        Arrays.sort(order, (a, b) -> Long.compare((long) wage[a] * quality[b], (long) wage[b] * quality[a]));

        PriorityQueue<Integer> heap = new PriorityQueue<>(Collections.reverseOrder()); // qualities in the group
        long totalQuality = 0;
        double best = Double.MAX_VALUE;
        for (int i : order) {
            heap.offer(quality[i]);
            totalQuality += quality[i];
            if (heap.size() > k) totalQuality -= heap.poll(); // drop the largest quality
            if (heap.size() == k) {
                // Worker i has the largest ratio so far and sets the pay rate.
                best = Math.min(best, (double) (totalQuality * wage[i]) / quality[i]);
            }
        }
        return best;
    }
}
