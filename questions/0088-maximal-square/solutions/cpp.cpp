class Solution {
public:
    int maximalSquare(vector<vector<string>>& matrix) {
        int cols = matrix[0].size();
        vector<int> side(cols + 1, 0);
        int best = 0;
        for (const auto& row : matrix) {
            int prevDiag = 0;
            for (int c = 1; c <= cols; c++) {
                int above = side[c];
                if (row[c - 1] == "1") {
                    side[c] = 1 + min({above, side[c - 1], prevDiag});
                    best = max(best, side[c]);
                } else {
                    side[c] = 0;
                }
                prevDiag = above;
            }
        }
        return best * best;
    }
};
