class Solution {
public:
    int numIslands(vector<vector<string>>& grid) {
        if (grid.empty()) return 0;
        int rows = (int)grid.size(), cols = (int)grid[0].size();
        vector<vector<bool>> seen(rows, vector<bool>(cols, false));
        const int dirs[4][2] = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        int count = 0;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (grid[r][c] != "1" || seen[r][c]) continue;
                count++;
                seen[r][c] = true;
                vector<pair<int, int>> stack = {{r, c}};
                while (!stack.empty()) {
                    auto [i, j] = stack.back();
                    stack.pop_back();
                    for (auto& d : dirs) {
                        int ni = i + d[0], nj = j + d[1];
                        if (ni >= 0 && ni < rows && nj >= 0 && nj < cols
                                && grid[ni][nj] == "1" && !seen[ni][nj]) {
                            seen[ni][nj] = true;
                            stack.push_back({ni, nj});
                        }
                    }
                }
            }
        }
        return count;
    }
};
