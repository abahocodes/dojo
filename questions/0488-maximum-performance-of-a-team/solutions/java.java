class Solution {
    public int maxPerformance(int n, int[] speed, int[] efficiency, int k) {
        Integer[] order = new Integer[n];
        for (int i = 0; i < n; i++) order[i] = i;
        Arrays.sort(order, (a, b) -> Integer.compare(efficiency[b], efficiency[a]));

        PriorityQueue<Integer> heap = new PriorityQueue<>(); // speeds in the current team
        long totalSpeed = 0;
        long best = 0; // up to 10^18: fits in a long
        for (int i : order) {
            // efficiency[i] is the smallest efficiency so far: it is the team minimum.
            heap.offer(speed[i]);
            totalSpeed += speed[i];
            if (heap.size() > k) totalSpeed -= heap.poll();
            best = Math.max(best, totalSpeed * efficiency[i]);
        }
        return (int) (best % 1_000_000_007L);
    }
}
