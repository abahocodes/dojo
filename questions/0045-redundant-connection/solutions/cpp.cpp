class Solution {
public:
    vector<int> findRedundantConnection(vector<vector<int>>& edges) {
        int n = edges.size();
        vector<int> parent(n + 1), size(n + 1, 1);
        iota(parent.begin(), parent.end(), 0);

        auto find = [&](int x) {
            while (parent[x] != x) {
                parent[x] = parent[parent[x]]; // path halving
                x = parent[x];
            }
            return x;
        };

        for (auto& e : edges) {
            int ra = find(e[0]), rb = find(e[1]);
            // a and b were already connected: this edge closes the cycle.
            if (ra == rb) return {e[0], e[1]};
            if (size[ra] < size[rb]) swap(ra, rb);
            parent[rb] = ra;
            size[ra] += size[rb];
        }
        return {};
    }
};
