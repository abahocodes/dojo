class Solution {
public:
    vector<int> regionSums(vector<vector<int>>& matrix, vector<vector<int>>& queries) {
        int m = matrix.size(), n = matrix[0].size();
        // pre[i][j] = sum of matrix[0..i-1][0..j-1]
        vector<vector<int>> pre(m + 1, vector<int>(n + 1, 0));
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                pre[i + 1][j + 1] = matrix[i][j] + pre[i][j + 1] + pre[i + 1][j] - pre[i][j];
            }
        }
        vector<int> result;
        result.reserve(queries.size());
        for (auto& q : queries) {
            int r1 = q[0], c1 = q[1], r2 = q[2], c2 = q[3];
            result.push_back(pre[r2 + 1][c2 + 1] - pre[r1][c2 + 1] - pre[r2 + 1][c1] + pre[r1][c1]);
        }
        return result;
    }
};
