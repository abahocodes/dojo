class Solution {
public:
    int networkDelayTime(vector<vector<int>>& times, int n, int k) {
        vector<vector<pair<int, int>>> graph(n + 1);
        for (auto& t : times) graph[t[0]].push_back({t[1], t[2]});

        vector<int> dist(n + 1, -1);
        // Min-heap of (time, node).
        priority_queue<pair<int, int>, vector<pair<int, int>>, greater<>> heap;
        heap.push({0, k});
        while (!heap.empty()) {
            auto [d, node] = heap.top();
            heap.pop();
            if (dist[node] != -1) continue;  // stale entry: already settled with a smaller time
            dist[node] = d;
            for (auto [next, w] : graph[node]) {
                if (dist[next] == -1) heap.push({d + w, next});
            }
        }

        int best = 0;
        for (int i = 1; i <= n; i++) {
            if (dist[i] == -1) return -1;
            best = max(best, dist[i]);
        }
        return best;
    }
};
