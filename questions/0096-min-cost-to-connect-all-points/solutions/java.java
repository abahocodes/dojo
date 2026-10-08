class Solution {
    public int minCostConnectPoints(int[][] points) {
        int n = points.length;
        int[] best = new int[n]; // cheapest known edge from the tree to each point
        Arrays.fill(best, Integer.MAX_VALUE);
        boolean[] inTree = new boolean[n];
        best[0] = 0;
        int total = 0;
        for (int step = 0; step < n; step++) {
            int u = -1, cost = Integer.MAX_VALUE;
            for (int v = 0; v < n; v++) {
                if (!inTree[v] && best[v] < cost) {
                    u = v;
                    cost = best[v];
                }
            }
            inTree[u] = true;
            total += cost;
            int ux = points[u][0], uy = points[u][1];
            for (int v = 0; v < n; v++) {
                if (!inTree[v]) {
                    int d = Math.abs(points[v][0] - ux) + Math.abs(points[v][1] - uy);
                    if (d < best[v]) best[v] = d;
                }
            }
        }
        return total;
    }
}
