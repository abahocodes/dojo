class Solution {
    public int[] minInterval(int[][] intervals, int[] queries) {
        int[][] sorted = intervals.clone();
        Arrays.sort(sorted, (a, b) -> Integer.compare(a[0], b[0]));
        Integer[] order = new Integer[queries.length];
        for (int i = 0; i < order.length; i++) order[i] = i;
        Arrays.sort(order, (a, b) -> Integer.compare(queries[a], queries[b]));

        int[] answer = new int[queries.length];
        Arrays.fill(answer, -1);
        // Min-heap of {length, right end} for intervals that start at or before q.
        PriorityQueue<int[]> heap = new PriorityQueue<>(
            (a, b) -> a[0] != b[0] ? Integer.compare(a[0], b[0]) : Integer.compare(a[1], b[1]));
        int i = 0;
        for (int qi : order) {
            int q = queries[qi];
            while (i < sorted.length && sorted[i][0] <= q) {
                heap.add(new int[] {sorted[i][1] - sorted[i][0] + 1, sorted[i][1]});
                i++;
            }
            // Intervals that end before q can never contain a later (larger) query.
            while (!heap.isEmpty() && heap.peek()[1] < q) heap.poll();
            if (!heap.isEmpty()) answer[qi] = heap.peek()[0];
        }
        return answer;
    }
}
