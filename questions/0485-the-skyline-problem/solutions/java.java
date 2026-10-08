class Solution {
    public int[][] getSkyline(int[][] buildings) {
        // Events {x, -height, right}: starts carry -height, ends carry 0.
        // Sorted by x, then starts before ends, tallest start first.
        int[][] events = new int[2 * buildings.length][];
        int e = 0;
        for (int[] b : buildings) {
            events[e++] = new int[] {b[0], -b[2], b[1]};
            events[e++] = new int[] {b[1], 0, 0};
        }
        Arrays.sort(events, (a, b) -> a[0] != b[0] ? Integer.compare(a[0], b[0]) : Integer.compare(a[1], b[1]));

        // Max-heap of {height, right} for buildings that may still stand.
        PriorityQueue<int[]> live = new PriorityQueue<>((a, b) -> Integer.compare(b[0], a[0]));
        List<int[]> result = new ArrayList<>();
        for (int[] ev : events) {
            int x = ev[0];
            // Lazily drop buildings that ended at or before x.
            while (!live.isEmpty() && live.peek()[1] <= x) live.poll();
            if (ev[1] != 0) live.offer(new int[] {-ev[1], ev[2]});
            // The first event at x already settles the height at x.
            int current = live.isEmpty() ? 0 : live.peek()[0];
            if (result.isEmpty() || result.get(result.size() - 1)[1] != current) {
                result.add(new int[] {x, current});
            }
        }
        return result.toArray(new int[0][]);
    }
}
