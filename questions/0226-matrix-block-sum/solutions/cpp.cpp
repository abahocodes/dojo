class Solution {
public:
    vector<vector<int>> matrixBlockSum(vector<vector<int>>& mat, int k) {
        int m = mat.size(), n = mat[0].size();
        vector<vector<int>> pre(m + 1, vector<int>(n + 1, 0));
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                pre[i + 1][j + 1] = mat[i][j] + pre[i][j + 1] + pre[i + 1][j] - pre[i][j];
            }
        }
        vector<vector<int>> result(m, vector<int>(n));
        for (int i = 0; i < m; i++) {
            int r1 = max(0, i - k), r2 = min(m, i + k + 1);
            for (int j = 0; j < n; j++) {
                int c1 = max(0, j - k), c2 = min(n, j + k + 1);
                result[i][j] = pre[r2][c2] - pre[r1][c2] - pre[r2][c1] + pre[r1][c1];
            }
        }
        return result;
    }
};
