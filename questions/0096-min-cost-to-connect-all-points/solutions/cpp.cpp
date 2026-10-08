class Solution {
public:
    int minCostConnectPoints(vector<vector<int>>& points) {
        int n = points.size();
        vector<int> best(n, INT_MAX); // cheapest known edge from the tree to each point
        vector<bool> inTree(n, false);
        best[0] = 0;
        int total = 0;
        for (int step = 0; step < n; step++) {
            int u = -1, cost = INT_MAX;
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
                    int d = abs(points[v][0] - ux) + abs(points[v][1] - uy);
                    if (d < best[v]) best[v] = d;
                }
            }
        }
        return total;
    }
};
