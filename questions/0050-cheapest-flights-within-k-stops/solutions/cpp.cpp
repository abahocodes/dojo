class Solution {
public:
    int findCheapestPrice(int n, vector<vector<int>>& flights, int src, int dst, int k) {
        const int INF = INT_MAX;
        vector<int> cost(n, INF);
        cost[src] = 0;
        // Round i relaxes every flight once, allowing paths of up to i flights.
        // At most k stops means at most k + 1 flights.
        for (int round = 0; round <= k; round++) {
            vector<int> next = cost;  // read last round's costs so one round adds one flight
            for (auto& f : flights) {
                int u = f[0], v = f[1], price = f[2];
                if (cost[u] != INF && cost[u] + price < next[v]) next[v] = cost[u] + price;
            }
            cost = move(next);
        }
        return cost[dst] == INF ? -1 : cost[dst];
    }
};
