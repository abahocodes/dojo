class Solution {
public:
    vector<int> kWeakestRows(vector<vector<int>>& mat, int k) {
        int rows = mat.size();
        vector<pair<int, int>> ranked;  // (soldiers, row index)
        ranked.reserve(rows);
        for (int i = 0; i < rows; i++) {
            // Rows are 1s then 0s: the first 0 marks the soldier count.
            int soldiers = lower_bound(mat[i].begin(), mat[i].end(), 0, greater<int>()) - mat[i].begin();
            ranked.push_back({soldiers, i});
        }
        sort(ranked.begin(), ranked.end());
        vector<int> result;
        for (int i = 0; i < k; i++) result.push_back(ranked[i].second);
        return result;
    }
};
