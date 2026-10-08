class Solution {
public:
    int equalPairs(vector<vector<int>>& grid) {
        int n = grid.size();
        map<vector<int>, int> rows;
        for (auto& row : grid) rows[row]++;
        int pairs = 0;
        vector<int> col(n);
        for (int c = 0; c < n; c++) {
            for (int r = 0; r < n; r++) col[r] = grid[r][c];
            auto it = rows.find(col);
            if (it != rows.end()) pairs += it->second;
        }
        return pairs;
    }
};
