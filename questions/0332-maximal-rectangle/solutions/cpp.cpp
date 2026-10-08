class Solution {
public:
    int maximalRectangle(vector<string>& matrix) {
        int cols = matrix[0].size();
        vector<int> heights(cols + 1, 0);  // heights[cols] stays 0
        vector<int> stk;
        stk.reserve(cols + 1);
        int best = 0;
        for (const string& row : matrix) {
            for (int c = 0; c < cols; c++) {
                heights[c] = row[c] == '1' ? heights[c] + 1 : 0;
            }
            stk.clear();
            for (int i = 0; i <= cols; i++) {
                while (!stk.empty() && heights[stk.back()] >= heights[i]) {
                    int h = heights[stk.back()];
                    stk.pop_back();
                    int left = stk.empty() ? -1 : stk.back();
                    best = max(best, h * (i - left - 1));
                }
                stk.push_back(i);
            }
        }
        return best;
    }
};
