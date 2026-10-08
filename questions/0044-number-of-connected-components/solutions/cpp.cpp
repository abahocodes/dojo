class Solution {
public:
    int countComponents(int n, vector<vector<int>>& edges) {
        vector<int> parent(n), size(n, 1);
        iota(parent.begin(), parent.end(), 0);

        auto find = [&](int x) {
            while (parent[x] != x) {
                parent[x] = parent[parent[x]]; // path halving
                x = parent[x];
            }
            return x;
        };

        int components = n;
        for (auto& e : edges) {
            int ra = find(e[0]), rb = find(e[1]);
            if (ra == rb) continue;
            if (size[ra] < size[rb]) swap(ra, rb);
            parent[rb] = ra;
            size[ra] += size[rb];
            components--;
        }
        return components;
    }
};
