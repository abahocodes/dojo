class Solution {
public:
    int numSubmatrixSumTarget(vector<vector<int>>& matrix, int target) {
        int rows = matrix.size();
        int cols = matrix[0].size();
        int count = 0;
        unordered_map<int, int> seen;
        seen.reserve(cols * 2 + 1);
        for (int top = 0; top < rows; top++) {
            vector<int> col(cols, 0);
            for (int bottom = top; bottom < rows; bottom++) {
                for (int c = 0; c < cols; c++) col[c] += matrix[bottom][c];
                seen.clear();
                seen[0] = 1;
                int s = 0;
                for (int v : col) {
                    s += v;
                    auto it = seen.find(s - target);
                    if (it != seen.end()) count += it->second;
                    seen[s]++;
                }
            }
        }
        return count;
    }
};
