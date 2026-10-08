class Solution {
public:
    int longestIncreasingPath(vector<vector<int>>& matrix) {
        int rows = matrix.size(), cols = matrix[0].size();
        // (height, r, c) by decreasing height
        vector<tuple<int, int, int>> order;
        order.reserve(rows * cols);
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) order.emplace_back(matrix[r][c], r, c);
        }
        sort(order.begin(), order.end(), greater<>());

        vector<vector<int>> best(rows, vector<int>(cols, 1));
        static const int dr[] = {1, -1, 0, 0};
        static const int dc[] = {0, 0, 1, -1};
        int answer = 1;
        for (const auto& [height, r, c] : order) {
            int length = 1;
            for (int d = 0; d < 4; d++) {
                int nr = r + dr[d], nc = c + dc[d];
                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && matrix[nr][nc] > height) {
                    length = max(length, best[nr][nc] + 1);
                }
            }
            best[r][c] = length;
            answer = max(answer, length);
        }
        return answer;
    }
};
