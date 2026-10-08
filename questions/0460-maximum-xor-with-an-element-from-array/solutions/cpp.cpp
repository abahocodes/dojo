class Solution {
public:
    vector<int> maximizeXor(vector<int>& nums, vector<vector<int>>& queries) {
        const int BITS = 30;
        vector<int> sorted = nums;
        sort(sorted.begin(), sorted.end());
        vector<int> order(queries.size());
        iota(order.begin(), order.end(), 0);
        sort(order.begin(), order.end(), [&](int a, int b) { return queries[a][1] < queries[b][1]; });

        vector<array<int, 2>> child(sorted.size() * BITS + 1, {0, 0});
        int nodes = 1;
        vector<int> answer(queries.size(), -1);
        size_t j = 0;

        for (int qi : order) {
            int x = queries[qi][0], limit = queries[qi][1];
            while (j < sorted.size() && sorted[j] <= limit) {
                int v = sorted[j], node = 0;
                for (int b = BITS - 1; b >= 0; b--) {
                    int bit = (v >> b) & 1;
                    if (child[node][bit] == 0) child[node][bit] = nodes++;
                    node = child[node][bit];
                }
                j++;
            }
            if (j == 0) continue;
            int node = 0, best = 0;
            for (int b = BITS - 1; b >= 0; b--) {
                int want = ((x >> b) & 1) ^ 1;
                if (child[node][want] != 0) {
                    best |= 1 << b;
                    node = child[node][want];
                } else {
                    node = child[node][want ^ 1];
                }
            }
            answer[qi] = best;
        }
        return answer;
    }
};
