class Solution {
    vector<int> parent;

    int find(int x) {
        while (parent[x] != x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    }

public:
    bool validTree(int n, vector<vector<int>>& edges) {
        if ((int)edges.size() != n - 1) return false;
        parent.resize(n);
        for (int i = 0; i < n; i++) parent[i] = i;
        for (const vector<int>& e : edges) {
            int ra = find(e[0]), rb = find(e[1]);
            if (ra == rb) return false;
            parent[ra] = rb;
        }
        return true;
    }
};
