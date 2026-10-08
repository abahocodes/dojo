class Solution {
    public int findCheapestPrice(int n, int[][] flights, int src, int dst, int k) {
        final int INF = Integer.MAX_VALUE;
        int[] cost = new int[n];
        Arrays.fill(cost, INF);
        cost[src] = 0;
        // Round i relaxes every flight once, allowing paths of up to i flights.
        // At most k stops means at most k + 1 flights.
        for (int round = 0; round <= k; round++) {
            int[] next = cost.clone(); // read last round's costs so one round adds one flight
            for (int[] f : flights) {
                int u = f[0], v = f[1], price = f[2];
                if (cost[u] != INF && cost[u] + price < next[v]) next[v] = cost[u] + price;
            }
            cost = next;
        }
        return cost[dst] == INF ? -1 : cost[dst];
    }
}
