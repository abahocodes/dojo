class Solution {
public:
    vector<vector<int>> diagonalSort(vector<vector<int>>& mat) {
        int m = mat.size(), n = mat[0].size();
        vector<vector<int>> res = mat;
        auto sortFrom = [&](int si, int sj) {
            int length = min(m - si, n - sj);
            vector<int> values(length);
            for (int k = 0; k < length; k++) values[k] = res[si + k][sj + k];
            sort(values.begin(), values.end());
            for (int k = 0; k < length; k++) res[si + k][sj + k] = values[k];
        };
        for (int i = 0; i < m; i++) sortFrom(i, 0);
        for (int j = 1; j < n; j++) sortFrom(0, j);
        return res;
    }
};
