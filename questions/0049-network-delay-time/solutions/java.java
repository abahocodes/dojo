class Solution {
    public int networkDelayTime(int[][] times, int n, int k) {
        List<List<int[]>> graph = new ArrayList<>();
        for (int i = 0; i <= n; i++) graph.add(new ArrayList<>());
        for (int[] t : times) graph.get(t[0]).add(new int[]{t[1], t[2]});

        int[] dist = new int[n + 1];
        Arrays.fill(dist, -1);
        PriorityQueue<int[]> heap = new PriorityQueue<>((a, b) -> Integer.compare(a[0], b[0]));
        heap.add(new int[]{0, k});
        while (!heap.isEmpty()) {
            int[] top = heap.poll();
            int d = top[0], node = top[1];
            if (dist[node] != -1) continue; // stale entry: already settled with a smaller time
            dist[node] = d;
            for (int[] edge : graph.get(node)) {
                if (dist[edge[0]] == -1) heap.add(new int[]{d + edge[1], edge[0]});
            }
        }

        int best = 0;
        for (int i = 1; i <= n; i++) {
            if (dist[i] == -1) return -1;
            best = Math.max(best, dist[i]);
        }
        return best;
    }
}
