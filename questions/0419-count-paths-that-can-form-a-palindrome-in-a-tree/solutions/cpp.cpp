class Solution {
public:
    long long countPalindromePaths(vector<int>& parent, string& s) {
        int n = parent.size();
        vector<vector<int>> children(n);
        for (int v = 1; v < n; v++) children[parent[v]].push_back(v);
        vector<int> mask(n, 0), order{0};
        order.reserve(n);
        for (size_t i = 0; i < order.size(); i++) {
            int v = order[i];
            for (int c : children[v]) {
                mask[c] = mask[v] ^ (1 << (s[c] - 'a'));
                order.push_back(c);
            }
        }
        unordered_map<int, int> seen;
        seen.reserve(n * 2);
        long long total = 0;
        for (int v = 0; v < n; v++) {
            int m = mask[v];
            auto it = seen.find(m);
            if (it != seen.end()) total += it->second;
            for (int b = 0; b < 26; b++) {
                auto jt = seen.find(m ^ (1 << b));
                if (jt != seen.end()) total += jt->second;
            }
            seen[m]++;
        }
        return total;
    }
};
