class Solution {
public:
    vector<int> lcaQueries(vector<int>& parent, vector<vector<int>>& queries) {
        int n = parent.size();
        vector<vector<int>> children(n);
        int root = 0;
        for (int v = 0; v < n; v++) {
            if (parent[v] == -1) root = v;
            else children[parent[v]].push_back(v);
        }

        vector<int> depth(n, 0);
        vector<int> order;
        order.reserve(n);
        order.push_back(root);
        for (size_t head = 0; head < order.size(); head++) {
            int u = order[head];
            for (int c : children[u]) {
                depth[c] = depth[u] + 1;
                order.push_back(c);
            }
        }

        int log = 1;
        while ((1 << log) < n) log++;
        vector<vector<int>> up(log, vector<int>(n));
        for (int v = 0; v < n; v++) up[0][v] = parent[v] == -1 ? root : parent[v];
        for (int k = 1; k < log; k++) {
            for (int v = 0; v < n; v++) up[k][v] = up[k - 1][up[k - 1][v]];
        }

        vector<int> answers;
        answers.reserve(queries.size());
        for (auto& q : queries) {
            int u = q[0], v = q[1];
            if (depth[u] < depth[v]) swap(u, v);
            int diff = depth[u] - depth[v];
            for (int k = 0; diff > 0; k++, diff >>= 1) {
                if (diff & 1) u = up[k][u];
            }
            if (u != v) {
                for (int k = log - 1; k >= 0; k--) {
                    if (up[k][u] != up[k][v]) {
                        u = up[k][u];
                        v = up[k][v];
                    }
                }
                u = up[0][u];
            }
            answers.push_back(u);
        }
        return answers;
    }
};
